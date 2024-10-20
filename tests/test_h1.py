import numpy as np
import pytest
from histr import h1

class TestH1:
    def test_works_no_args(self):
        h = h1([1, 2, 2.04])
        assert h.bin_edges == pytest.approx([1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9, 2.0, 2.1])
        assert h.bin_contents == [1.0] + [0.0] * 9 + [2.0]

    def test_works_bin_width(self):
        h = h1([1, 2, 2.04], bin_width=0.4)
        assert h.bin_edges == pytest.approx([0.8, 1.2, 1.6, 2.0, 2.4])
        assert h.bin_contents == [1.0, 0.0, 0.0, 2.0]

    def test_works_bin_edges(self):
        h = h1([1, 2, 2.04], bin_edges=[1.0, 1.2, 1.4, 1.6, 1.8, 2.0, 2.2])
        assert h.bin_edges == pytest.approx([1.0, 1.2, 1.4, 1.6, 1.8, 2.0, 2.2])
        assert h.bin_contents == [1.0] + [0.0] * 4 + [2.0]




        