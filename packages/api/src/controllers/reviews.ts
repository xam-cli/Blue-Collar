import type { Request, Response } from 'express'
import * as reviewService from '../services/review.service.js'
import { handleError } from '../utils/handleError.js'

/**
 * POST /api/workers/:id/reviews
 * Create a review for a worker. Authenticated users only; one review per worker.
 */
export async function createReview(req: Request, res: Response) {
  try {
    const { rating, comment } = req.body
    const review = await reviewService.createReview(req.params.id, req.user!.id, Number(rating), comment)
    return res.status(201).json({ data: review, status: 'success', code: 201 })
  } catch (err) {
    return handleError(res, err)
  }
}

/**
 * GET /api/workers/:id/reviews
 * List reviews for a worker (public, paginated). Includes averageRating and reviewCount.
 */
export async function listReviews(req: Request, res: Response) {
  try {
    const page = Number(req.query.page ?? 1)
    const limit = Number(req.query.limit ?? 20)
    const result = await reviewService.listReviews(req.params.id, page, limit)
    return res.json({ ...result, status: 'success', code: 200 })
  } catch (err) {
    return handleError(res, err)
  }
}
