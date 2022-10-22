/*
  Warnings:

  - The primary key for the `instances` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - You are about to drop the column `ownerId` on the `instances` table. All the data in the column will be lost.
  - Changed the type of `instance_id` on the `dashboards` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.
  - Added the required column `owner_id` to the `instances` table without a default value. This is not possible if the table is not empty.
  - Changed the type of `id` on the `instances` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.

*/
-- DropForeignKey
ALTER TABLE "dashboards" DROP CONSTRAINT "dashboards_instance_id_fkey";

-- DropForeignKey
ALTER TABLE "instances" DROP CONSTRAINT "instances_ownerId_fkey";

-- AlterTable
ALTER TABLE "dashboards" DROP COLUMN "instance_id",
ADD COLUMN     "instance_id" BIGINT NOT NULL;

-- AlterTable
ALTER TABLE "instances" DROP CONSTRAINT "instances_pkey",
DROP COLUMN "ownerId",
ADD COLUMN     "owner_id" BIGINT NOT NULL,
DROP COLUMN "id",
ADD COLUMN     "id" BIGINT NOT NULL,
ADD CONSTRAINT "instances_pkey" PRIMARY KEY ("id");

-- AddForeignKey
ALTER TABLE "instances" ADD CONSTRAINT "instances_owner_id_fkey" FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "dashboards" ADD CONSTRAINT "dashboards_instance_id_fkey" FOREIGN KEY ("instance_id") REFERENCES "instances"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
