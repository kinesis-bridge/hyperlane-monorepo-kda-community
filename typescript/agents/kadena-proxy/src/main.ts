import { AllExceptionsFilter } from './all-exceptions.filter';
import { AppModule } from './app.module';
import { LogLevel, ValidationPipe } from '@nestjs/common';
import { NestFactory } from '@nestjs/core';
import { DocumentBuilder, SwaggerModule } from '@nestjs/swagger';

async function bootstrap() {
  const logLevels = (process.env.LOG_LEVEL || 'log').split(',') as LogLevel[];
  const app = await NestFactory.create(AppModule, { logger: logLevels });
  const config = new DocumentBuilder()
    .setTitle('Kadena proxy')
    .setDescription('The kadena-proxy API description')
    .setVersion('0.1.0')
    .addTag('kadena')
    .build();
  const document = SwaggerModule.createDocument(app, config);
  SwaggerModule.setup('api', app, document);

  app.useGlobalPipes(
    new ValidationPipe({
      transform: true,
      whitelist: true,
      forbidNonWhitelisted: true,
      disableErrorMessages: false,
      validationError: {
        target: false,
        value: false,
      },
    }),
  );
  app.useGlobalFilters(new AllExceptionsFilter());

  const server = app.getHttpServer();
  server.keepAliveTimeout = 60000;

  await app.listen(3000);
}
bootstrap();
