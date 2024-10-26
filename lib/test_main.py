import sqlite3
import os
import unittest
from extract import extract
from transform_load import load
from query import read_query, update_query, delete_query, sorting_Change


class TestMainFunctions(unittest.TestCase):
    dataset_path = "data/murder_2015_final.csv"

    @classmethod
    def setUpClass(cls):
        # Ensure the data file exists
        if not os.path.exists(cls.dataset_path):
            url = "https://raw.githubusercontent.com/fivethirtyeight/data/master/murder_2016/murder_2015_final.csv"
            extract(url, cls.dataset_path)

    def setUp(self):
        # Delete existing database to ensure a clean test environment
        if os.path.exists("Murder2015.db"):
            os.remove("Murder2015.db")

        # Load data into the database
        load(self.dataset_path)

        # Check if "Chicago" is in the database; if not, insert it manually
        conn = sqlite3.connect("Murder2015.db")
        cursor = conn.cursor()

        # Check if Chicago is in the database
        cursor.execute("SELECT * FROM Murder2015 WHERE city = 'Chicago'")
        chicago = cursor.fetchone()

        # If Chicago is missing, insert it
        if not chicago:
            cursor.execute(
                "INSERT INTO Murder2015 (city, state, murders_2014, murders_2015, change) VALUES (?, ?, ?, ?, ?)",
                ("Chicago", "Illinois", 411, 478, 67),
            )
            conn.commit()

        conn.close()

    def test_extract(self):
        url = "https://raw.githubusercontent.com/fivethirtyeight/data/master/murder_2016/murder_2015_final.csv"
        file_path = "data/murder_2015_final.csv"

        # Test extraction
        result = extract(url, file_path)
        self.assertEqual(result, file_path, "Failed to extract data")

        # Verify the file was downloaded
        self.assertTrue(
            os.path.exists(file_path), "File was not found after extraction"
        )

    def test_load(self):
        # Ensure the file exists in the expected directory
        self.assertTrue(
            os.path.exists(self.dataset_path),
            "Dataset file not found in 'data' directory",
        )

        # Test load function
        result = load(self.dataset_path)
        self.assertEqual(
            result, "Murder2015.db", "Failed to load data into the database"
        )

        # Confirm the database file is created
        self.assertTrue(
            os.path.exists("Murder2015.db"), "Database file was not created"
        )

    def test_read_query(self):
        # Test reading the query (this prints the top 5 rows)
        result = read_query()
        self.assertEqual(
            result, "Read Success", "read_query function did not execute as expected"
        )

    def test_update_query(self):
        # Test updating the record for Chicago
        result = update_query()
        self.assertEqual(result, "Update Success", "Failed to update database record")

        # Verify the update
        conn = sqlite3.connect("Murder2015.db")
        cursor = conn.cursor()
        cursor.execute("SELECT change FROM Murder2015 WHERE city = 'Chicago'")
        change = cursor.fetchone()
        self.assertIsNotNone(change, "No record found for 'Chicago'")
        self.assertEqual(
            change[0], 60, "Chicago's 'change' value was not updated to 60"
        )
        conn.close()

    def test_delete_query(self):
        # Test deleting the record for Chicago
        result = delete_query()
        self.assertEqual(
            result, "Delete Success", "Failed to delete record from database"
        )

        # Verify the deletion
        conn = sqlite3.connect("Murder2015.db")
        cursor = conn.cursor()
        cursor.execute("SELECT COUNT(*) FROM Murder2015 WHERE city = 'Chicago'")
        count = cursor.fetchone()[0]
        self.assertEqual(count, 0, "Record for 'Chicago' was not deleted")
        conn.close()

    def test_sorting_change(self):
        # Test sorting function
        result = sorting_Change()
        self.assertEqual(
            result,
            "Sort Success",
            "sorting_Change function did not execute as expected",
        )

    @classmethod
    def tearDownClass(cls):
        # Clean up after tests
        if os.path.exists("Murder2015.db"):
            os.remove("Murder2015.db")


if __name__ == "__main__":
    unittest.main()
