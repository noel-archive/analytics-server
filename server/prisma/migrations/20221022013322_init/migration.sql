-- CreateTable
CREATE TABLE "User" (
    "display_name" TEXT NOT NULL,
    "id" BIGINT NOT NULL,

    CONSTRAINT "User_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Instance" (
    "grpc_endpoint" TEXT NOT NULL,
    "service_token" TEXT NOT NULL,
    "display_name" TEXT NOT NULL,
    "ownerId" BIGINT NOT NULL,
    "icon" TEXT,
    "id" TEXT NOT NULL,

    CONSTRAINT "Instance_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Dashboard" (
    "display_name" TEXT NOT NULL,
    "instance_id" TEXT NOT NULL,
    "data" JSONB NOT NULL,
    "id" BIGINT NOT NULL,

    CONSTRAINT "Dashboard_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "Instance" ADD CONSTRAINT "Instance_ownerId_fkey" FOREIGN KEY ("ownerId") REFERENCES "User"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Dashboard" ADD CONSTRAINT "Dashboard_instance_id_fkey" FOREIGN KEY ("instance_id") REFERENCES "Instance"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
