import { Router } from 'express'
import {
  listWorkers,
  createWorker,
  showWorker,
  updateWorker,
  deleteWorker,
  toggleActivation,
} from '../controllers/workers.js'
import { authenticate, authorize } from '../middleware/auth.js'

const router = Router()

router.get('/', listWorkers)
router.get('/:id', showWorker)
router.post('/', authenticate, authorize('curator'), createWorker)
router.post('/:id', authenticate, authorize('curator'), updateWorker) // method-spoofed PUT
router.delete('/:id', authenticate, authorize('curator'), deleteWorker)
router.patch('/:id/toggle', authenticate, authorize('curator'), toggleActivation)

export default router
