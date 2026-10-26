ALTER TABLE shop.orders ADD note clob;
ALTER TABLE shop.orders ADD placed_at timestamp with time zone DEFAULT systimestamp NOT NULL;
CREATE TABLE shop.shipments (
  id raw(16) DEFAULT sys_guid() PRIMARY KEY,
  order_id raw(16) NOT NULL REFERENCES shop.orders(id),
  method varchar2(32) NOT NULL CONSTRAINT ck_shipments_method CHECK (method IN ('ground', 'air'))
);
