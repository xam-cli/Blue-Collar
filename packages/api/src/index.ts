// Entry point for BlueCollar API
import express from 'express'
import cors from 'cors'
import helmet from 'helmet'
import authRoutes from './routes/auth.js'
import categoryRoutes from './routes/categories.js'
import workerRoutes from './routes/workers.js'

const app = express()
const PORT = process.env.PORT || 3000

// Apply Helmet for HTTP security headers
// strict CSP since we only serve JSON, not HTML
app.use(helmet({
  contentSecurityPolicy: {
    directives: {
      defaultSrc: ["'none'"],
      baseUri: ["'none'"],
      formAction: ["'none'"],
      frameAncestors: ["'none'"],
    },
  },
}))

app.use(cors())
app.use(express.json())
app.use(express.urlencoded({ extended: true }))

app.use('/api/auth', authRoutes)
app.use('/api/categories', categoryRoutes)
app.use('/api/workers', workerRoutes)

app.get('/health', (_req, res) => {
  res.json({ status: 'ok', service: 'bluecollar-api' })
})

app.listen(PORT, () => {
  console.log(`BlueCollar API running on port ${PORT}`)
})

export default app
