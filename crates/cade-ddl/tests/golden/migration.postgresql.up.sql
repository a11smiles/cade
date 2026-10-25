CREATE TYPE shop.ship_method AS ENUM ('ground', 'air');
ALTER TABLE shop.orders ADD COLUMN note text;
ALTER TABLE shop.orders ADD COLUMN placed_at timestamptz NOT NULL DEFAULT now();
CREATE TABLE shop.shipments (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  order_id uuid NOT NULL REFERENCES shop.orders(id),
  method shop.ship_method NOT NULL
);
