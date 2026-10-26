CREATE TABLE [shop].[customers] (
  [id] uniqueidentifier PRIMARY KEY DEFAULT newsequentialid(),
  [email] nvarchar(320) COLLATE Latin1_General_CI_AS NOT NULL,
  [bio] nvarchar(max),
  [visits] int NOT NULL DEFAULT 0,
  [lifetime_cents] bigint NOT NULL,
  [active] bit NOT NULL,
  [joined_on] date NOT NULL,
  [created_at] datetimeoffset NOT NULL DEFAULT sysdatetimeoffset(),
  [avatar] varbinary(max)
);

CREATE TABLE [shop].[orders] (
  [id] uniqueidentifier PRIMARY KEY DEFAULT newsequentialid(),
  [customer_id] uniqueidentifier NOT NULL REFERENCES [shop].[customers]([id]),
  [total] decimal(12,2) NOT NULL,
  [state] nvarchar(32) NOT NULL
);
