CREATE TABLE shop.customers (
  id raw(16) DEFAULT sys_guid() PRIMARY KEY,
  email varchar2(320) NOT NULL,
  bio clob,
  visits number(10) DEFAULT 0 NOT NULL,
  lifetime_cents number(19) NOT NULL,
  active number(1) NOT NULL,
  joined_on date NOT NULL,
  created_at timestamp with time zone DEFAULT systimestamp NOT NULL,
  avatar blob
);

CREATE TABLE shop.orders (
  id raw(16) DEFAULT sys_guid() PRIMARY KEY,
  customer_id raw(16) NOT NULL REFERENCES shop.customers(id),
  total number(12,2) NOT NULL,
  state varchar2(32) NOT NULL CONSTRAINT ck_orders_state CHECK (state IN ('open', 'paid'))
);
