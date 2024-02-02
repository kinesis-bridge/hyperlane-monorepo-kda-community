import { Injectable, NestMiddleware } from '@nestjs/common';
import { Request, Response, NextFunction } from 'express';

@Injectable()
export class TrimHostMiddleware implements NestMiddleware {
  use(req: Request, res: Response, next: NextFunction) {
    if (typeof req.query.host === 'string') {
      req.query.host = req.query.host.replace(/\/$/, ''); // kadena.js requires host without trailing slash although it's a valid URL
    }
    next();
  }
}
