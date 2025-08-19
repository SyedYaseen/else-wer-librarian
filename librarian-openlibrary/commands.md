split -l 200000 ol_dump_works.txt
sqlx migrate revert --all   # Roll back everything
sqlx migrate run            # Re-run all migrations