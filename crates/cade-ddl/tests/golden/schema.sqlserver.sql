CREATE TABLE [shop].[customers] (
  [id] uniqueidentifier CONSTRAINT [df_customers_id] DEFAULT newsequentialid() PRIMARY KEY,
  [email] nvarchar(320) COLLATE Latin1_General_CI_AS NOT NULL,
  [bio] nvarchar(max),
  [visits] int CONSTRAINT [df_customers_visits] DEFAULT 0 NOT NULL,
  [lifetime_cents] bigint NOT NULL,
  [active] bit NOT NULL,
  [joined_on] date NOT NULL,
  [created_at] datetimeoffset CONSTRAINT [df_customers_created_at] DEFAULT sysdatetimeoffset() NOT NULL,
  [avatar] varbinary(max)
);

CREATE TABLE [shop].[orders] (
  [id] uniqueidentifier CONSTRAINT [df_orders_id] DEFAULT newsequentialid() PRIMARY KEY,
  [customer_id] uniqueidentifier NOT NULL REFERENCES [shop].[customers]([id]),
  [total] decimal(12,2) NOT NULL,
  [state] nvarchar(32) NOT NULL CONSTRAINT [ck_orders_state] CHECK ([state] IN ('open', 'paid'))
);
