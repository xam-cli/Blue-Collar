import type { Request, Response } from 'express'
import { db } from '../db.js'

export async function listCategories(_req: Request, res: Response) {
  const categories = await db.category.findMany()
  return res.json({ data: categories, status: 'success', code: 200 })
}

export async function getCategory(req: Request, res: Response) {
  const category = await db.category.findUnique({ where: { id: req.params.id } })
  if (!category) return res.status(404).json({ status: 'error', message: 'Not found', code: 404 })
  return res.json({ data: category, status: 'success', code: 200 })
}
