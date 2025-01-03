# Monte Carlo Permutation

Monte Carlo Permutation method is an alternative method for testing 
significance and robustness of a trading strategy’s performance.

H<sub>0</sub>: All rules tested have output values that are randomly correlated 
with future market behaviour.
</br>
H<sub>A</sub>: The rule back-tested profitability is the result of an 
informative pairing of its long and short positions with one-day-forward price 
change.

- Uses different data than bootstrap and it tests a different formulation of 
H<sub>0</sub>.
- The p-value of Monte Carlo Permutation gives almost identical result of 
bootstrapping.


# Example
    
    from monte_carlo_permutation import mcp

    p_value, sample_dis = mcp(signals, returns)