from histr import h1

class TestH1:
    def test_works_on_lists(self):
        h = h1([1, 2, 3])
        assert h.bin_edges() == [1, 2, 3]
        assert h.bin_contents() == [1, 1, 1]
        