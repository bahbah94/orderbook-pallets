// "use client"

// import { useEffect, useMemo, useRef, useState } from "react"

// export type Candle = {
//   time: number // unix seconds (UDF-compatible)
//   open: number
//   high: number
//   low: number
//   close: number
//   volume?: number
// }

// export type UseSimulatedCandlesOpts = {
//   symbol: string
//   candleMs?: number      // default 1m
//   history?: number       // default 400
//   tickMs?: number        // default 1000ms
//   volatility?: number    // default 0.002 (0.2% per tick band)
//   startPrice?: number    // default derived from symbol hash
// }

// function mulberry32(seed: number) {
//   let t = seed >>> 0
//   return () => {
//     t += 0x6D2B79F5
//     let r = Math.imul(t ^ (t >>> 15), 1 | t)
//     r ^= r + Math.imul(r ^ (r >>> 7), 61 | r)
//     return ((r ^ (r >>> 14)) >>> 0) / 4294967296
//   }
// }

// function hashStr(s: string) {
//   let h = 2166136261 >>> 0
//   for (let i = 0; i < s.length; i++) {
//     h ^= s.charCodeAt(i)
//     h = Math.imul(h, 16777619)
//   }
//   return h >>> 0
// }

// export function useSimulatedCandles({
//   symbol,
//   candleMs = 60_000,
//   history = 400,
//   tickMs = 1000,
//   volatility = 0.002,
//   startPrice,
// }: UseSimulatedCandlesOpts) {
//   const seed = useMemo(() => hashStr(symbol || "SYMBOL"), [symbol])
//   const rng = useMemo(() => mulberry32(seed), [seed])

//   const [candles, setCandles] = useState<Candle[]>([])

//   const lastPriceRef = useRef<number>(startPrice ?? 100)
//   const currentStartRef = useRef<number>(Math.floor(Date.now() / 1000))

//   // 🔁 (Re)build initial history whenever resolution / symbol / params change
//   useEffect(() => {
//     const now = Date.now()
//     const start = Math.floor((now - history * candleMs) / 1000)
//     const base = startPrice ?? (100 + (rng() - 0.5) * 40)

//     const arr: Candle[] = []
//     let lastClose = base

//     for (let i = 0; i < history; i++) {
//       const t = start + Math.floor((i * candleMs) / 1000)
//       const drift = (rng() - 0.5) * 2 * volatility * 20
//       const open = lastClose
//       const close = Math.max(0.0001, open * (1 + drift))
//       const spread = Math.abs(close - open)
//       const high = Math.max(open, close) + spread * (0.2 + rng() * 0.8)
//       const low = Math.min(open, close) - spread * (0.2 + rng() * 0.8)
//       const vol = 10 + rng() * 90
//       arr.push({ time: t, open, high, low, close, volume: vol })
//       lastClose = close
//     }

//     setCandles(arr)
//     lastPriceRef.current = lastClose
//     currentStartRef.current = arr[arr.length - 1]?.time ?? Math.floor(Date.now() / 1000)
//   }, [symbol, candleMs, history, volatility, startPrice, rng])

//   // Align the current candle bucket when candleMs changes
//   useEffect(() => {
//     const nowMs = Date.now()
//     const aligned = nowMs - (nowMs % candleMs)
//     currentStartRef.current = Math.floor(aligned / 1000)
//   }, [candleMs])

//   // Live ticking within the latest candle / rolling to next candle
//   useEffect(() => {
//     const timer = setInterval(() => {
//       const nowMs = Date.now()
//       const candleStartMs = currentStartRef.current * 1000
//       const nextBucket = nowMs >= candleStartMs + candleMs

//       const lp = lastPriceRef.current
//       const move = (rng() - 0.5) * 2 * volatility
//       const next = Math.max(0.0001, lp * (1 + move))
//       lastPriceRef.current = next

//       setCandles(prev => {
//         const last = prev[prev.length - 1]
//         if (!last) {
//           const t = Math.floor(nowMs / 1000)
//           return [{ time: t, open: next, high: next, low: next, close: next, volume: 0 }]
//         }

//         if (nextBucket) {
//           const start = candleStartMs + candleMs
//           currentStartRef.current = Math.floor(start / 1000)
//           const open = last.close
//           const close = next
//           const high = Math.max(open, close)
//           const low = Math.min(open, close)
//           const vol = (last.volume ?? 50) * (0.9 + rng() * 0.2)
//           return [
//             ...prev,
//             {
//               time: Math.floor(start / 1000), // ⬅️ still unix seconds (UDF style)
//               open,
//               high,
//               low,
//               close,
//               volume: vol,
//             },
//           ]
//         } else {
//           const updated = { ...last }
//           updated.close = next
//           updated.high = Math.max(updated.high, next)
//           updated.low = Math.min(updated.low, next)
//           updated.volume = (updated.volume ?? 0) + (0.2 + rng() * 0.8)
//           const copy = prev.slice(0, -1)
//           copy.push(updated)
//           return copy
//         }
//       })
//     }, tickMs)

//     return () => clearInterval(timer)
//   }, [tickMs, candleMs, rng, volatility])

//   return candles
// }

"use client"

import { useEffect, useMemo, useRef, useState } from "react"
import {
  IndexerRestClient,
  IndexerWebSocketClient,
  type CandleUpdate,
  type TradingViewBar,
} from "@/lib/indexer" // <- change to "frontend/lib/indexer" if needed

export type Candle = TradingViewBar

export type UseSimulatedCandlesOpts = {
  symbol: string
  candleMs?: number      // default 1m
  history?: number       // number of candles (default 400)
  tickMs?: number        // kept for backwards compat, ignored now
  volatility?: number    // kept for backwards compat, ignored now
  startPrice?: number    // kept for backwards compat, ignored now
}

// map your candleMs to indexer interval strings
function candleMsToInterval(ms: number): string {
  switch (ms) {
    case 60_000:
      return "1m"
    case 5 * 60_000:
      return "5m"
    case 15 * 60_000:
      return "15m"
    case 30 * 60_000:
      return "30m"
    case 60 * 60_000:
      return "1h"
    case 4 * 60 * 60_000:
      return "4h"
    case 24 * 60 * 60_000:
      return "1d"
    case 7 * 24 * 60 * 60_000:
      return "1w"
    default:
      return "1m"
  }
}

export function useSimulatedCandles({
  symbol,
  candleMs = 60_000,
  history = 400,
}: UseSimulatedCandlesOpts) {
  const [candles, setCandles] = useState<Candle[]>([])

  const wsClientRef = useRef<IndexerWebSocketClient | null>(null)
  const restClientRef = useRef<IndexerRestClient | null>(null)

  const interval = useMemo(() => candleMsToInterval(candleMs), [candleMs])

  useEffect(() => {
    const httpUrl =
      process.env.NEXT_PUBLIC_INDEXER_HTTP_URL ??
      process.env.NEXT_PUBLIC_INDEXER_URL ??
      "http://localhost:8081"

    const wsUrl =
      process.env.NEXT_PUBLIC_INDEXER_WS_URL ??
      "ws://localhost:8081/ws/market"

    if (!httpUrl || !wsUrl) {
      console.error(
        "[useSimulatedCandles] Missing NEXT_PUBLIC_INDEXER_HTTP_URL / NEXT_PUBLIC_INDEXER_WS_URL",
    )
    return
  }

    const restClient = new IndexerRestClient(httpUrl)
    restClientRef.current = restClient

    const wsClient = new IndexerWebSocketClient(wsUrl, {
      orderbook: false,
      ohlcv: true,
      symbol,
      timeframes: [interval],
    })
    wsClientRef.current = wsClient

    let unsubCandle: (() => void) | undefined
    let cancelled = false

    const setup = async () => {
      try {
        const nowSec = Math.floor(Date.now() / 1000)
        const secondsPerBar = Math.floor(candleMs / 1000)
        const start_time = nowSec - history * secondsPerBar
        const end_time = nowSec

        // 1) initial history from REST
        const bars = await restClient.getCandlesAsTvBars({
          symbol,
          start_time,
          end_time,
          interval,
        })
        console.log(`candles ${bars}`);
        if (!cancelled) {
          setCandles(bars)
        }

        // 2) live updates from WebSocket
        await wsClient.connect()
        if (cancelled) return

        unsubCandle = wsClient.onCandle((candle: CandleUpdate) => {
          if (candle.s !== symbol || candle.i !== interval) return

          const bar: Candle = {
            time: Math.floor(candle.t / 1000), // same as getCandlesAsTvBars
            open: parseFloat(candle.o),
            high: parseFloat(candle.h),
            low: parseFloat(candle.l),
            close: parseFloat(candle.c),
            volume: parseFloat(candle.v),
          }

          setCandles(prev => {
            if (prev.length === 0) return [bar]

            const last = prev[prev.length - 1]
            // same candle bucket → update it
            if (last.time === bar.time) {
              const merged: Candle = {
                ...last,
                high: Math.max(last.high, bar.high),
                low: Math.min(last.low, bar.low),
                close: bar.close,
                volume: (last.volume ?? 0) + (bar.volume ?? 0),
              }
              return [...prev.slice(0, -1), merged]
            }

            // newer candle → append
            if (bar.time > last.time) {
              return [...prev, bar]
            }

            // out-of-order / old update → ignore
            return prev
          })
        })
      } catch (error) {
        console.error("[useSimulatedCandles] Failed to init indexer candles:", error)
      }
    }

    setup()

    return () => {
      cancelled = true
      if (unsubCandle) unsubCandle()
      wsClientRef.current?.disconnect()
      wsClientRef.current = null
    }
  }, [symbol, interval, candleMs, history])

  return candles
}
