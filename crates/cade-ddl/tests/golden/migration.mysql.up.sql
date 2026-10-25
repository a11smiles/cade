ALTER TABLE `shop`.`orders` ADD COLUMN `note` text;
ALTER TABLE `shop`.`orders` ADD COLUMN `placed_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP;
CREATE TABLE `shop`.`shipments` (
  `id` char(36) PRIMARY KEY DEFAULT (uuid()),
  `order_id` char(36) NOT NULL,
  `method` enum('ground', 'air') NOT NULL,
  CONSTRAINT `fk_shipments_order_id` FOREIGN KEY (`order_id`) REFERENCES `shop`.`orders`(`id`)
);
