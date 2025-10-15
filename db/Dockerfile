# we base you dockerfile on debian with node preinstalled
FROM node:24.10.0-trixie-slim AS base

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable

WORKDIR /app

COPY package.json ./
COPY pnpm-lock.yaml ./
COPY pnpm-workspace.yaml ./
RUN pnpm install --frozen-lockfile

RUN pnpm dlx prisma
COPY . .

# start the application
CMD ["pnpm", "run", "deploy"]