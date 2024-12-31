import {
  ExceptionFilter,
  Catch,
  ArgumentsHost,
  HttpException,
  HttpStatus,
  BadRequestException,
} from '@nestjs/common';
import { ValidationError } from 'class-validator';

@Catch()
export class AllExceptionsFilter implements ExceptionFilter {
  catch(exception: any, host: ArgumentsHost) {
    const ctx = host.switchToHttp();
    const response = ctx.getResponse();
    const request = ctx.getRequest();

    let status = HttpStatus.INTERNAL_SERVER_ERROR;
    let message = exception.message || 'Internal server error';

    if (exception instanceof HttpException) {
      status = exception.getStatus();
      message = exception.getResponse();
    } else if (this.isValidationError(exception)) {
      status = HttpStatus.BAD_REQUEST;
      message = this.formatValidationErrors(exception);
    }

    response.status(status).json({
      statusCode: status,
      timestamp: new Date().toISOString(),
      path: request.url,
      message: message,
    });
  }

  private isValidationError(exception: any): boolean {
    return (
      Array.isArray(exception) &&
      exception.every((item) => item instanceof ValidationError)
    );
  }

  private formatValidationErrors(errors: ValidationError[]): any {
    return errors.map((error) => ({
      property: error.property,
      constraints: error.constraints,
      children: error.children
        ? this.formatValidationErrors(error.children)
        : undefined,
    }));
  }
}
