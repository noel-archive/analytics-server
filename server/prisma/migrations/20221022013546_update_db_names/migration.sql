/*
  Warnings:

  - You are about to drop the `Dashboard` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `Instance` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `User` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropForeignKey
ALTER TABLE "Dashboard" DROP CONSTRAINT "Dashboard_instance_id_fkey";

-- DropForeignKey
ALTER TABLE "Instance" DROP CONSTRAINT "Instance_ownerId_fkey";

-- DropTable
DROP TABLE "Dashboard";

-- DropTable
DROP TABLE "Instance";

-- DropTable
DROP TABLE "User";

-- CreateTable
CREATE TABLE "users" (
    "display_name" TEXT NOT NULL,
    "id" BIGINT NOT NULL,

    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "instances" (
    "grpc_endpoint" TEXT NOT NULL,
    "service_token" TEXT NOT NULL,
    "display_name" TEXT NOT NULL,
    "ownerId" BIGINT NOT NULL,
    "icon" TEXT,
    "id" TEXT NOT NULL,

    CONSTRAINT "instances_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "dashboards" (
    "display_name" TEXT NOT NULL,
    "instance_id" TEXT NOT NULL,
    "data" JSONB NOT NULL,
    "id" BIGINT NOT NULL,

    CONSTRAINT "dashboards_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "instances" ADD CONSTRAINT "instances_ownerId_fkey" FOREIGN KEY ("ownerId") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "dashboards" ADD CONSTRAINT "dashboards_instance_id_fkey" FOREIGN KEY ("instance_id") REFERENCES "instances"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
