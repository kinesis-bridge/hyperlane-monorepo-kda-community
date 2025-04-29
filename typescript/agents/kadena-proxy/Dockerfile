FROM node:23-slim AS builder

ENV NODE_ENV=build

USER node
WORKDIR /home/node

# Copy package.json with correct ownership
COPY --chown=node:node package.json yarn.lock ./

# Install all dependencies, including development dependencies
RUN yarn install

# Copy the rest of the application files with correct ownership
COPY --chown=node:node . .
# Build the application
RUN yarn build

FROM node:23-slim

# Use apt-get instead of apk to install jq
USER root
RUN apt-get update && apt-get install -y jq && apt-get clean

ENV NODE_ENV=production

USER node
WORKDIR /home/node

# Copy package.json with correct ownership
COPY --chown=node:node package.json yarn.lock ./

# Install only production dependencies
RUN yarn install --production

# Copy built application from the builder stage
COPY --from=builder --chown=node:node /home/node/dist/ ./dist/

ENTRYPOINT ["node"]
CMD ["dist/main.js"]