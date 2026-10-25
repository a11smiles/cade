CREATE TYPE shop.order_state AS ENUM ('open', 'paid');

CREATE TABLE shop.customers (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  email citext NOT NULL,
  bio text,
  visits int NOT NULL DEFAULT 0,
  lifetime_cents bigint NOT NULL,
  active boolean NOT NULL,
  joined_on date NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  avatar bytea
);

CREATE TABLE shop.orders (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  customer_id uuid NOT NULL REFERENCES shop.customers(id),
  total decimal(12,2) NOT NULL,
  state shop.order_state NOT NULL
);
