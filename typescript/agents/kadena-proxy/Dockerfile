FROM node:20-alpine as builder

ENV NODE_ENV build

USER node
WORKDIR /home/node

# Copy package.json with correct ownership
COPY --chown=node:node package.json ./
# Install all dependencies, including development dependencies
RUN yarn install

# Copy the rest of the application files with correct ownership
COPY --chown=node:node . .
# Build the application
RUN yarn build

FROM node:20-alpine

ENV NODE_ENV production

USER node
WORKDIR /home/node

# Copy package.json with correct ownership
COPY --chown=node:node package.json ./
# Install only production dependencies
RUN yarn install --production

# Copy built application from the builder stage
COPY --from=builder --chown=node:node /home/node/dist/ ./dist/

CMD ["node", "dist/main.js"]