-- One cart per user, the user must exist, and one row per product in a cart.
--
-- NOTE: the live `carts` table still has `product_id` and `quantity` columns
-- from the old design (before cart_items existed). `product_id` is NOT NULL
-- with no default, so `INSERT INTO carts (user_id)` fails today. They are
-- dropped here. Any data in them is lost -- it is leftover from the old shape.

BEGIN;

-- 1. Merge duplicate carts onto the oldest cart per user.
CREATE TEMP TABLE keep AS
    SELECT user_id, MIN(id) AS cart_id
    FROM carts
    GROUP BY user_id;

-- 1a. Product already in the kept cart: add the quantity there and drop the dup row.
UPDATE cart_items k
SET quantity = k.quantity + d.quantity
FROM cart_items d
JOIN carts c ON c.id = d.cart_id
JOIN keep ON keep.user_id = c.user_id
WHERE d.cart_id <> keep.cart_id
  AND k.cart_id = keep.cart_id
  AND k.product_id = d.product_id;

DELETE FROM cart_items d
USING carts c, keep
WHERE c.id = d.cart_id
  AND keep.user_id = c.user_id
  AND d.cart_id <> keep.cart_id
  AND EXISTS (
      SELECT 1 FROM cart_items k
      WHERE k.cart_id = keep.cart_id AND k.product_id = d.product_id
  );

-- 1b. Everything else: move it onto the kept cart.
UPDATE cart_items ci
SET cart_id = keep.cart_id
FROM carts c
JOIN keep ON keep.user_id = c.user_id
WHERE ci.cart_id = c.id
  AND ci.cart_id <> keep.cart_id;

DELETE FROM carts c
WHERE c.id <> (SELECT MIN(id) FROM carts WHERE user_id = c.user_id);

-- 2. Drop carts pointing at users who no longer exist.
DELETE FROM carts c
WHERE NOT EXISTS (SELECT 1 FROM users u WHERE u.id = c.user_id);

-- 3. Merge duplicate rows of the same product inside one cart.
WITH merged AS (
    SELECT MIN(id) AS keep_id, SUM(quantity) AS total
    FROM cart_items
    GROUP BY cart_id, product_id
)
UPDATE cart_items ci
SET quantity = merged.total
FROM merged
WHERE ci.id = merged.keep_id;

DELETE FROM cart_items ci
WHERE ci.id <> (
    SELECT MIN(id) FROM cart_items
    WHERE cart_id = ci.cart_id AND product_id = ci.product_id
);

-- 4. Remove the leftover columns from the old cart design.
ALTER TABLE carts DROP COLUMN IF EXISTS product_id;
ALTER TABLE carts DROP COLUMN IF EXISTS quantity;

-- 5. The actual rules.
ALTER TABLE carts ADD CONSTRAINT carts_user_id_key UNIQUE (user_id);

ALTER TABLE carts
    ADD CONSTRAINT carts_user_id_fkey
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;

-- (cart_id, product_id) is already UNIQUE via `unique_cart_product`.

COMMIT;
