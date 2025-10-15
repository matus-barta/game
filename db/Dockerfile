# we base you dockerfile on debian with node preinstalled
FROM node:24.10.0-trixie AS base

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable

WORKDIR /app

COPY package.json ./
COPY pnpm-lock.yaml ./
RUN pnpm dlx prisma
RUN pnpm install --frozen-lockfile

COPY schema.prisma ./
COPY migrations ./migrations

# start the application
CMD ["pnpm", "run", "deploy"]