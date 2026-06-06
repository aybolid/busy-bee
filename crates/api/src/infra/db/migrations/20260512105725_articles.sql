CREATE TABLE rss_feeds (
  id BLOB NOT NULL PRIMARY KEY,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  --
  status TEXT NOT NULL CHECK (status IN ('new', 'healthy', 'error')),
  error_reason TEXT, -- Exists only if status == error. Enforced by constraint below.
  url TEXT NOT NULL UNIQUE,
  max_concurrent_requests INTEGER NOT NULL,
  fetch_interval_seconds INTEGER NOT NULL,
  --
  CONSTRAINT check_error_reason CHECK (
    (
      status = 'error'
      AND error_reason IS NOT NULL
    )
    OR (
      status = 'new'
      AND error_reason IS NULL
    )
    OR (
      status = 'healthy'
      AND error_reason IS NULL
    )
  )
) WITHOUT ROWID;

CREATE TRIGGER trigger_rss_feeds_updated_at AFTER
UPDATE ON rss_feeds FOR EACH ROW WHEN OLD.updated_at = NEW.updated_at BEGIN
UPDATE rss_feeds
SET
  updated_at = CURRENT_TIMESTAMP
WHERE
  id = NEW.id;

END;

CREATE TABLE articles (
  id BLOB NOT NULL PRIMARY KEY,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  --
  status TEXT NOT NULL CHECK (
    status IN ('new', 'pending', 'processed', 'error')
  ),
  error_reason TEXT,
  --
  rss_feed_id BLOB NOT NULL REFERENCES rss_feeds (id) ON DELETE CASCADE ON UPDATE CASCADE,
  --
  title TEXT NOT NULL,
  byline TEXT,
  content TEXT NOT NULL,
  text_content TEXT NOT NULL,
  length INTEGER NOT NULL,
  excerpt TEXT,
  site_name TEXT,
  dir TEXT,
  lang TEXT,
  published_time DATETIME,
  modified_time DATETIME,
  image TEXT,
  favicon TEXT,
  url TEXT NOT NULL UNIQUE,
  --
  CONSTRAINT check_error_reason CHECK (
    (
      status = 'error'
      AND error_reason IS NOT NULL
    )
    OR (
      status = 'new'
      AND error_reason IS NULL
    )
    OR (
      status = 'pending'
      AND error_reason IS NULL
    )
    OR (
      status = 'processed'
      AND error_reason IS NULL
    )
  )
) WITHOUT ROWID;

CREATE TRIGGER trigger_articles_updated_at AFTER
UPDATE ON articles FOR EACH ROW WHEN OLD.updated_at = NEW.updated_at BEGIN
UPDATE articles
SET
  updated_at = CURRENT_TIMESTAMP
WHERE
  id = NEW.id;

END;

CREATE TABLE outputs (
  id BLOB NOT NULL PRIMARY KEY,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  --
  article_id BLOB REFERENCES articles (id) ON DELETE SET NULL ON UPDATE CASCADE,
  --
  user_context TEXT,
  text TEXT NOT NULL,
  --
  model TEXT NOT NULL,
  usage TEXT NOT NULL -- JSON
) WITHOUT ROWID;

CREATE TRIGGER trigger_outputs_updated_at AFTER
UPDATE ON outputs FOR EACH ROW WHEN OLD.updated_at = NEW.updated_at BEGIN
UPDATE outputs
SET
  updated_at = CURRENT_TIMESTAMP
WHERE
  id = NEW.id;

END;
