// Import modules (environment variables will be loaded by database connection)
import express from 'express';
import cors from 'cors';
import helmet from 'helmet';
import multer from 'multer';
import { analyzeImageRoute } from './routes/ai.js';
import authRoutes from './routes/auth.js';
import subscriptionRoutes from './routes/subscription.js';

const app = express();
const PORT = process.env.PORT || 3001;

// Security middleware
app.use(helmet());

// CORS configuration for Vercel frontend
app.use(cors({
  origin: [
    'http://localhost:5173', // Local development
    'http://localhost:3000', // Website server
    'http://localhost:3001', // Local development
    'http://localhost:8080', // Payments server
    process.env.FRONTEND_URL || '', // Production Vercel URL
  ],
  credentials: true
}));

// Body parsing middleware
app.use(express.json({ limit: '10mb' }));
app.use(express.urlencoded({ extended: true, limit: '10mb' }));

// Configure multer for image uploads
const upload = multer({
  storage: multer.memoryStorage(),
  limits: {
    fileSize: 10 * 1024 * 1024 // 10MB limit
  }
});

// Health check endpoint
app.get('/health', (req, res) => {
  res.json({ 
    status: 'healthy', 
    timestamp: new Date().toISOString(),
    service: 'FrameSense API'
  });
});

// Authentication routes
app.use('/api/auth', authRoutes);

// Subscription management routes
app.use('/api', subscriptionRoutes);

// AI analysis endpoint
app.post('/api/analyze', upload.single('image'), analyzeImageRoute);

// Health check for optimization services
app.get('/api/health/optimizations', async (req, res) => {
  try {
    const { AIProcessor } = await import('./services/ai-processor.js');
    const healthStatus = await AIProcessor.healthCheck();
    
    res.json({
      status: healthStatus.overall ? 'healthy' : 'degraded',
      services: healthStatus,
      timestamp: new Date().toISOString()
    });
  } catch (error: any) {
    res.status(500).json({
      status: 'error',
      message: error.message || 'Unknown error',
      timestamp: new Date().toISOString()
    });
  }
});

// Error handling middleware
app.use((err: any, req: express.Request, res: express.Response, next: express.NextFunction) => {
  console.error('Error:', err);
  res.status(500).json({ 
    error: 'Internal server error',
    message: process.env.NODE_ENV === 'development' ? err.message : 'Something went wrong'
  });
});

// 404 handler
app.use('*', (req, res) => {
  res.status(404).json({ error: 'Route not found' });
});

app.listen(PORT, () => {
  console.log(`🚀 FrameSense API running on port ${PORT}`);
  console.log(`🔗 Health check: http://localhost:${PORT}/health`);
  console.log(`🔐 Auth endpoints: http://localhost:${PORT}/api/auth/*`);
  console.log(`💳 Subscription endpoints: http://localhost:${PORT}/api/*`);
});

export default app; 