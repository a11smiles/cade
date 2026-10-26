ALTER TABLE [shop].[orders] ADD [note] nvarchar(max);
ALTER TABLE [shop].[orders] ADD [placed_at] datetimeoffset CONSTRAINT [df_orders_placed_at] DEFAULT sysdatetimeoffset() NOT NULL;
CREATE TABLE [shop].[shipments] (
  [id] uniqueidentifier CONSTRAINT [df_shipments_id] DEFAULT newsequentialid() PRIMARY KEY,
  [order_id] uniqueidentifier NOT NULL REFERENCES [shop].[orders]([id]),
  [method] nvarchar(32) NOT NULL CONSTRAINT [ck_shipments_method] CHECK ([method] IN ('ground', 'air'))
);
