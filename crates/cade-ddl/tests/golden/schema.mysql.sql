CREATE TABLE `shop`.`customers` (
  `id` char(36) PRIMARY KEY DEFAULT (uuid()),
  `email` varchar(320) NOT NULL,
  `bio` text,
  `visits` int NOT NULL DEFAULT 0,
  `lifetime_cents` bigint NOT NULL,
  `active` tinyint(1) NOT NULL,
  `joined_on` date NOT NULL,
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `avatar` blob
);

CREATE TABLE `shop`.`orders` (
  `id` char(36) PRIMARY KEY DEFAULT (uuid()),
  `customer_id` char(36) NOT NULL REFERENCES `shop`.`customers`(`id`),
  `total` decimal(12,2) NOT NULL,
  `state` enum('open', 'paid') NOT NULL
);
