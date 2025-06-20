-- Add up migration script here

-- infohashes
CREATE TABLE IF NOT EXISTS infohashes (
  infohash            BLOB            PRIMARY KEY,
  name                VARCHAR(4096)   NOT NULL,
  length              INTEGER         NOT NULL,
  chunk_size          INTEGER         NOT NULL,
  chunk_count         INTEGER         NOT NULL,
  creation_timestamp  DATETIME        NOT NULL,
  signature           TEXT            NOT NULL
);

-- Chunks
CREATE TABLE IF NOT EXISTS chunks (
  chunk_hash           BLOB      PRIMARY KEY,
  k                    INTEGER   NOT NULL,
  m                    INTEGER   NOT NULL,
  chunk_size           INTEGER   NOT NULL,
  padlen               INTEGER   NOT NULL,
  original_chunk_size  INTEGER   NOT NULL
);

-- Chunk‑tracker mapping 
CREATE TABLE IF NOT EXISTS tracker_chunks (
  infohash   BLOB    NOT NULL  REFERENCES infohashes(infohash),
  chunk_idx  INTEGER NOT NULL,
  chunk_hash BLOB    NOT NULL  REFERENCES chunks(chunk_hash),
  PRIMARY KEY (infohash, chunk_idx)
);

-- Pieces
CREATE TABLE IF NOT EXISTS pieces (
  piece_hash   BLOB      PRIMARY KEY,
  piece_size   INTEGER   NOT NULL,
  piece_type   INTEGER   NOT NULL,
  miners       TEXT      NOT NULL
);

-- Piece‑chunk mapping 
CREATE TABLE IF NOT EXISTS chunk_pieces (
  chunk_hash  BLOB    NOT NULL  REFERENCES chunks(chunk_hash),
  piece_idx   INTEGER NOT NULL,
  piece_hash  BLOB    NOT NULL  REFERENCES pieces(piece_hash),
  PRIMARY KEY (chunk_hash, piece_idx)
);

-- Piece‑repair history
CREATE TABLE IF NOT EXISTS piece_repair_history (
  piece_repair_hash  BLOB      PRIMARY KEY,
  piece_hash         BLOB      NOT NULL,
  chunk_hash         BLOB      NOT NULL  REFERENCES chunks(chunk_hash),
  validator_id       INTEGER   NOT NULL,
  timestamp          DATETIME  NOT NULL,
  signature          TEXT      NOT NULL
);

-- Chunk‑challenge history
CREATE TABLE IF NOT EXISTS chunk_challenge_history (
  challenge_hash     BLOB      PRIMARY KEY,
  chunk_hash         BLOB      NOT NULL  REFERENCES chunks(chunk_hash),
  validator_id       INTEGER   NOT NULL,
  miners_challenged  TEXT      NOT NULL,
  miners_successful  TEXT      NOT NULL,
  piece_repair_hash      BLOB      REFERENCES piece_repair_history(piece_repair_hash),
  timestamp          DATETIME  NOT NULL,
  signature          TEXT      NOT NULL
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_chunks_by_hash     ON chunks(chunk_hash);
CREATE INDEX IF NOT EXISTS idx_pieces_by_hash     ON pieces(piece_hash);
CREATE INDEX IF NOT EXISTS idx_chunk_pieces_by_chunk ON chunk_pieces(chunk_hash);
CREATE INDEX IF NOT EXISTS idx_tracker_chunks_by_infohash ON tracker_chunks(infohash);
CREATE INDEX IF NOT EXISTS idx_piece_repair_by_piece ON piece_repair_history(piece_hash);
CREATE INDEX IF NOT EXISTS idx_chunk_challenge_by_chunk ON chunk_challenge_history(chunk_hash);
