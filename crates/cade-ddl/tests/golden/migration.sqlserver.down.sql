DROP TABLE [shop].[shipments];
ALTER TABLE [shop].[orders] DROP CONSTRAINT [df_orders_placed_at], COLUMN [placed_at];
ALTER TABLE [shop].[orders] DROP COLUMN [note];
