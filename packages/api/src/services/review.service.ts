import { db } from '../db.js'
import { AppError } from './AppError.js'

/**
 * Create a review for a worker. A user may only review a worker once.
 * @throws AppError 404 if worker not found
 * @throws AppError 409 if user already reviewed this worker
 */
export async function createReview(
  workerId: string,
  authorId: string,
  rating: number,
  comment?: string,
) {
  if (rating < 1 || rating > 5) throw new AppError('Rating must be between 1 and 5', 400)

  const worker = await db.worker.findUnique({ where: { id: workerId } })
  if (!worker) throw new AppError('Worker not found', 404)

  const existing = await db.review.findUnique({
    where: { authorId_workerId: { authorId, workerId } },
  })
  if (existing) throw new AppError('You have already reviewed this worker', 409)

  return db.review.create({
    data: { workerId, authorId, rating, comment },
    include: { author: { select: { id: true, firstName: true, lastName: true, avatar: true } } },
  })
}

/**
 * Return a paginated list of reviews for a worker, plus aggregate stats.
 */
export async function listReviews(workerId: string, page: number, limit: number) {
  const where = { workerId }
  const [reviews, total, agg] = await Promise.all([
    db.review.findMany({
      where,
      skip: (page - 1) * limit,
      take: limit,
      orderBy: { createdAt: 'desc' },
      include: { author: { select: { id: true, firstName: true, lastName: true, avatar: true } } },
    }),
    db.review.count({ where }),
    db.review.aggregate({ where, _avg: { rating: true } }),
  ])

  return {
    data: reviews,
    meta: { total, page, limit, pages: Math.ceil(total / limit) },
    averageRating: agg._avg.rating ? Math.round(agg._avg.rating * 10) / 10 : null,
    reviewCount: total,
  }
}
