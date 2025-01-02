from typing import Iterable

def mcp(signals: list[Iterable] | Iterable, returns: Iterable, m: int = 5000
        ) -> tuple[float, list[float]]:
    """Compute and return the p-value and sample distribution of Monte Carlo
    Permutation test.

    Procedures of Monte Carlo Permutation test:

    1) Shuffle market data (i.e. market returns or price difference that
       reflects the change in value of an asset). Market data does not need
       to be de-trended.

    2) Pair long/short signals with shuffled market data from step 1)
       without replacement.

    3) Compute the rules' returns by multiplying the signals with the
       shuffled market data.

    4) Compute the average of rules' returns.

    5) Select and store the max among all signals as sample distribution.

    6) Repeat steps 1) to 5) for `m` times, where `m` is a large number like
       5000.

    7) Compute the best expected returns by multiplying the raw market data
       with long/short signals. Then, use the best expected returns to
       compute the p-value with sampling distribution formed in step 6).

    Parameters
    ----------
    signals : Sized
        Trading signals (+1 for long and -1 for short).
    returns : Sized
        Returns of asset.
    m : int = 5000
        Number of Bootstrapping.

    Returns
    -------
    p_value : float
        p-value of Monte Carlo Permutation test.
    sample_distribution : np.ndarray
        Sample Distribution.

    Raises
    ------
    ValueError
        - If argument `signals` does not share the same length with argument
        `returns`.
        - If argument `m` is not an int.
        - If argument `signals` contains `np.nan`.
        - If argument `returns` contains `np.nan`.
    """