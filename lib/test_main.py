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
        # Ensure the dataset file exists by extracting if necessary
        if not os.path.exists(cls.dataset_path):
            url = "https://raw.githubusercontent.com/fivethirtyeight/data/master/murder_2016/murder_2015_final.csv"
            print("Extracting dataset...")
            extract(url, cls.dataset_path)

    def setUp(self):
        # Clean up existing database file to ensure a fresh test environment
        if os.path.exists("Murder2015.db"):
            os.remove("Murder2015.db")

        # Load data into the database
        load(self.dataset_path)

        # Ensure "Chicago" is in the database
        conn = sqlite3.connect("Murder2015.db")
        cursor = conn.cursor()
        cursor.execute("SELECT * FROM Murder2015 WHERE city = 'Chicago'")
        if cursor.fetchone() is None:
            print("Inserting 'Chicago' record...")
            cursor.execute(
                """
                INSERT INTO Murder2015 (city, state, murders_2014, murders_2015, change) 
                VALUES (?, ?, ?, ?, ?)",
                ("Chicago", "Illinois", 411, 478, 67),
                """
            )
            conn.commit()
        conn.close()

    def test_extract(self):
        url = "https://raw.githubusercontent.com/fivethirtyeight/data/master/murder_2016/murder_2015_final.csv"
        file_path = "data/murder_2015_final.csv"

        # Test extraction
        result = extract(url, file_path)
        self.assertEqual(
            result, file_path, "Expected file path after extraction does not match."
        )

        # Verify the file exists
        self.assertTrue(
            os.path.exists(file_path), "File was not found after extraction."
        )

    def test_load(self):
        # Verify the file exists
        self.assertTrue(
            os.path.exists(self.dataset_path),
            "Dataset file not found in 'data' directory.",
        )

        # Test load function
        result = load(self.dataset_path)
        self.assertEqual(
            result,
            "Murder2015.db",
            "Database file path after load does not match expected.",
        )

        # Confirm database creation
        self.assertTrue(
            os.path.exists("Murder2015.db"), "Database file was not created."
        )

    def test_read_query(self):
        result = read_query()
        self.assertEqual(
            result,
            "Read Success",
            "Failed to execute 'read_query' function successfully.",
        )

    def test_update_query(self):
        result = update_query()
        self.assertEqual(
            result, "Update Success", "Failed to update database record for 'Chicago'."
        )

        # Verify update effect
        conn = sqlite3.connect("Murder2015.db")
        cursor = conn.cursor()
        cursor.execute("SELECT change FROM Murder2015 WHERE city = 'Chicago'")
        change = cursor.fetchone()
        conn.close()

        self.assertIsNotNone(change, "No record found for 'Chicago' after update.")
        self.assertEqual(
            change[0], 60, "Chicago's 'change' value was not updated to 60."
        )

    def test_delete_query(self):
        result = delete_query()
        self.assertEqual(
            result, "Delete Success", "Failed to delete 'Chicago' record from database."
        )

        # Verify deletion
        conn = sqlite3.connect("Murder2015.db")
        cursor = conn.cursor()
        cursor.execute("SELECT COUNT(*) FROM Murder2015 WHERE city = 'Chicago'")
        count = cursor.fetchone()[0]
        conn.close()

        self.assertEqual(count, 0, "Record for 'Chicago' was not deleted.")

    def test_sorting_change(self):
        result = sorting_Change()
        self.assertEqual(
            result, "Sort Success", "Failed to sort records based on 'change' column."
        )

    @classmethod
    def tearDownClass(cls):
        # Clean up after all tests
        if os.path.exists("Murder2015.db"):
            os.remove("Murder2015.db")
        print("Database cleaned up after tests.")


if __name__ == "__main__":
    unittest.main()
