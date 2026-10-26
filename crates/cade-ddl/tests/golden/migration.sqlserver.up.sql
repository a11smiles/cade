ALTER TABLE [shop].[orders] ADD [note] nvarchar(max);
ALTER TABLE [shop].[orders] ADD [placed_at] datetimeoffset NOT NULL DEFAULT sysdatetimeoffset();
CREATE TABLE [shop].[shipments] (
  [id] uniqueidentifier PRIMARY KEY DEFAULT newsequentialid(),
  [order_id] uniqueidentifier NOT NULL REFERENCES [shop].[orders]([id]),
  [method] nvarchar(32) NOT NULL CONSTRAINT [ck_shipments_method] CHECK ([method] IN ('ground', 'air'))
);
