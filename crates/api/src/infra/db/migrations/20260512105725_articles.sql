CREATE TABLE rss_feeds (
  id BLOB NOT NULL PRIMARY KEY,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  --
  status TEXT NOT NULL CHECK (status IN ('healthy', 'error')),
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
      status = 'healthy'
      AND error_reason IS NULL
    )
  )
);

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
  url TEXT
);

CREATE TRIGGER trigger_articles_updated_at AFTER
UPDATE ON articles FOR EACH ROW WHEN OLD.updated_at = NEW.updated_at BEGIN
UPDATE articles
SET
  updated_at = CURRENT_TIMESTAMP
WHERE
  id = NEW.id;

END;

CREATE TABLE article_processing_outputs (
  id BLOB NOT NULL PRIMARY KEY,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  --
  article_id BLOB REFERENCES articles (id) ON DELETE SET NULL ON UPDATE CASCADE,
  --
  user_context TEXT,
  output_text TEXT NOT NULL,
  --
  model TEXT NOT NULL,
  prompt_tokens INTEGER,
  prompt_cache_creation_tokens INTEGER,
  prompt_cached_tokens INTEGER,
  prompt_audio_tokens INTEGER,
  completion_tokens INTEGER,
  completion_accepted_prediction_tokens INTEGER,
  completion_rejected_prediction_tokens INTEGER,
  completion_reasoning_tokens INTEGER,
  completion_audio_tokens INTEGER,
  total_tokens INTEGER
);

CREATE TRIGGER trigger_outputs_updated_at AFTER
UPDATE ON article_processing_outputs FOR EACH ROW WHEN OLD.updated_at = NEW.updated_at BEGIN
UPDATE article_processing_outputs
SET
  updated_at = CURRENT_TIMESTAMP
WHERE
  id = NEW.id;

END;
