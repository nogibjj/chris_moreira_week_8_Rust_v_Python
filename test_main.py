from main import main_results


def test_main_results():
    results = main_results()

    # Check if each function returns the expected result
    assert (
        results["extract_to"] == "data/Spotify_Most_Streamed_Songs.csv"
    ), "Extract did not return the expected path"
    assert (
        results["transform_db"] == "SpotifyDB.db"
    ), "Transform & Load did not return the expected database"
    assert (
        results["create"] == "Create Success"
    ), "query_create did not return the expected success message"
    assert (
        results["read"] == "Read Success"
    ), "query_read did not return the expected success message"
    assert (
        results["update"] == "Update Success"
    ), "query_update did not return the expected success message"
    assert (
        results["delete"] == "Delete Success"
    ), "query_delete did not return the expected success message"


if __name__ == "__main__":
    test_main_results()
    print("All tests passed!")
