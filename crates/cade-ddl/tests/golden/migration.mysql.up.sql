ALTER TABLE `shop`.`orders` ADD COLUMN `note` text;
ALTER TABLE `shop`.`orders` ADD COLUMN `placed_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP;
CREATE TABLE `shop`.`shipments` (
  `id` char(36) PRIMARY KEY DEFAULT (uuid()),
  `order_id` char(36) NOT NULL REFERENCES `shop`.`orders`(`id`),
  `method` enum('ground', 'air') NOT NULL
);
