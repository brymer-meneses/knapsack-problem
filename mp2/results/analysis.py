import numpy as np
import pandas as pd
import plotly.graph_objects as go
from plotly.subplots import make_subplots
from scipy import stats
from scipy.stats import f_oneway

# Load data
bottom_up = pd.read_csv("bottom_up.csv")
top_down_memoized = pd.read_csv("top_down_memoized.csv")
greatest_worth_first = pd.read_csv("greatest_worth_first.csv")
largest_value_first = pd.read_csv("largest_value_first.csv")
smallest_weight_first = pd.read_csv("smallest_weight_first.csv")
# After loading CSV files and before creating visualizations
# Extract n_values and averages for plotting
n_values = bottom_up['n'].values
bottom_up_avg = bottom_up['average'].values
top_down_avg = top_down_memoized['average'].values
cache_hits = top_down_memoized['average.3'].values  # Cache hits column
cache_misses = top_down_memoized['average.2'].values  # Cache misses columnb

# Calculate statistics for each algorithm
def calculate_stats(df, time_columns):
    stats_dict = {}
    for col in time_columns:
        stats_dict[f"{col}_mean"] = df[col].mean()
        stats_dict[f"{col}_std"] = df[col].std()
        stats_dict[f"{col}_cv"] = stats_dict[f"{col}_std"] / stats_dict[f"{col}_mean"]  # Coefficient of variation
    return stats_dict

# Create figures
def create_execution_time_comparison():
    fig = go.Figure()
    
    # Bottom-up
    fig.add_trace(go.Scatter(
        x=bottom_up['n'],
        y=bottom_up['average'],
        name='Bottom-up DP (Table)',
        line=dict(color='blue')
    ))
    
    # Top-down
    fig.add_trace(go.Scatter(
        x=top_down_memoized['n'],
        y=top_down_memoized['average'],
        name='Top-down DP (Table)',
        line=dict(color='red')
    ))
    
    # Greedy algorithms
    fig.add_trace(go.Scatter(
        x=greatest_worth_first['n'],
        y=greatest_worth_first['average'],
        name='Greatest Worth First',
        line=dict(color='green')
    ))
    
    fig.add_trace(go.Scatter(
        x=largest_value_first['n'],
        y=largest_value_first['average'],
        name='Largest Value First',
        line=dict(color='purple')
    ))
    
    fig.add_trace(go.Scatter(
        x=smallest_weight_first['n'],
        y=smallest_weight_first['average'],
        name='Smallest Weight First',
        line=dict(color='orange')
    ))
    
    fig.update_layout(
        title='Algorithm Execution Time Comparison',
        xaxis_title='Input Size (n)',
        yaxis_title='Execution Time (seconds)',
        template='plotly_white'
    )
    
    return fig

def create_dp_metrics_comparison():
    fig = make_subplots(
        rows=2, cols=2,
        subplot_titles=('Table Creation Time', 'Backtracking Time',
                       'Cache Performance (Top-down)', 'Efficiency Ratio')
    )
    
    # Table creation time comparison
    fig.add_trace(
        go.Scatter(x=bottom_up['n'], y=bottom_up['average'],
                  name='Bottom-up Table', line=dict(color='blue')),
        row=1, col=1
    )
    fig.add_trace(
        go.Scatter(x=top_down_memoized['n'], y=top_down_memoized['average'],
                  name='Top-down Table', line=dict(color='red')),
        row=1, col=1
    )
    
    # Backtracking time comparison
    fig.add_trace(
        go.Scatter(x=bottom_up['n'], y=bottom_up['average.1'],
                  name='Bottom-up Backtrack', line=dict(color='blue', dash='dash')),
        row=1, col=2
    )
    fig.add_trace(
        go.Scatter(x=top_down_memoized['n'], y=top_down_memoized['average.1'],
                  name='Top-down Backtrack', line=dict(color='red', dash='dash')),
        row=1, col=2
    )
    
    # Cache performance for top-down
    fig.add_trace(
        go.Scatter(x=top_down_memoized['n'], y=top_down_memoized['average.2'],
                  name='Cache Misses', line=dict(color='red')),
        row=2, col=1
    )
    fig.add_trace(
        go.Scatter(x=top_down_memoized['n'], y=top_down_memoized['average.3'],
                  name='Cache Hits', line=dict(color='green')),
        row=2, col=1
    )
    
    # Efficiency ratio (cache hits / total cache accesses)
    cache_efficiency = top_down_memoized['average.3'] / (top_down_memoized['average.2'] + top_down_memoized['average.3'])
    fig.add_trace(
        go.Scatter(x=top_down_memoized['n'], y=cache_efficiency,
                  name='Cache Efficiency', line=dict(color='purple')),
        row=2, col=2
    )
    
    fig.update_layout(height=800, title_text="Detailed Dynamic Programming Metrics")
    return fig

def create_greedy_comparison():
    fig = go.Figure()
    
    # Define colors with direct RGBA values
    colors = {
        'Greatest Worth First': 'rgba(0, 128, 0, 1)',      # green
        'Largest Value First': 'rgba(128, 0, 128, 1)',     # purple
        'Smallest Weight First': 'rgba(255, 165, 0, 1)'    # orange
    }
    
    algorithms = [
        (greatest_worth_first, 'Greatest Worth First'),
        (largest_value_first, 'Largest Value First'),
        (smallest_weight_first, 'Smallest Weight First')
    ]
    
    for df, name in algorithms:
        # Main line
        fig.add_trace(go.Scatter(
            x=df['n'],
            y=df['average'],
            name=name,
            line=dict(color=colors[name])
        ))
        
        # Add standard deviation bands
        std = df[['trial 1', 'trial 2', 'trial 3']].std(axis=1)
        fig.add_trace(go.Scatter(
            x=df['n'].tolist() + df['n'].tolist()[::-1],
            y=(df['average'] + std).tolist() + (df['average'] - std).tolist()[::-1],
            fill='toself',
            fillcolor=colors[name].replace('1)', '0.2)'),  # Make it transparent
            line=dict(color='rgba(0,0,0,0)'),
            showlegend=False,
            name=f'{name} Std Dev'
        ))
    
    fig.update_layout(
        title='Greedy Algorithms Comparison with Standard Deviation',
        xaxis_title='Input Size (n)',
        yaxis_title='Execution Time (seconds)',
        template='plotly_white'
    )
    
    return fig

# Calculate statistics
dp_stats = {
    'bottom_up': calculate_stats(bottom_up, ['average', 'average.1']),
    'top_down': calculate_stats(top_down_memoized, ['average', 'average.1', 'average.2', 'average.3'])
}

greedy_stats = {
    'greatest_worth': calculate_stats(greatest_worth_first, ['average']),
    'largest_value': calculate_stats(largest_value_first, ['average']),
    'smallest_weight': calculate_stats(smallest_weight_first, ['average'])
}

# Create and save plots
execution_comparison = create_execution_time_comparison()
dp_metrics = create_dp_metrics_comparison()
greedy_comparison = create_greedy_comparison()

execution_comparison.write_html("execution_comparison.html")
dp_metrics.write_html("dp_metrics.html")
greedy_comparison.write_html("greedy_comparison.html")

# execution_comparison.write_image("execution_comparison.png")
# dp_metrics.write_image("dp_metrics.png")
# greedy_comparison.write_image("greedy_comparison.png")

# Print statistical summary
print("\nStatistical Summary:")
print("\nDynamic Programming Algorithms:")
print("Bottom-up DP:")
print(f"Table Creation - Mean: {dp_stats['bottom_up']['average_mean']:.6f}s, Std: {dp_stats['bottom_up']['average_std']:.6f}s")
print(f"Backtracking - Mean: {dp_stats['bottom_up']['average.1_mean']:.6f}s, Std: {dp_stats['bottom_up']['average.1_std']:.6f}s")

print("\nTop-down DP:")
print(f"Table Creation - Mean: {dp_stats['top_down']['average_mean']:.6f}s, Std: {dp_stats['top_down']['average_std']:.6f}s")
print(f"Backtracking - Mean: {dp_stats['top_down']['average.1_mean']:.6f}s, Std: {dp_stats['top_down']['average.1_std']:.6f}s")
print(f"Cache Hits - Mean: {dp_stats['top_down']['average.3_mean']:.0f}, Std: {dp_stats['top_down']['average.3_std']:.0f}")
print(f"Cache Misses - Mean: {dp_stats['top_down']['average.2_mean']:.0f}, Std: {dp_stats['top_down']['average.2_std']:.0f}")

print("\nGreedy Algorithms:")
for name, stats in greedy_stats.items():
    print(f"\n{name.replace('_', ' ').title()}:")
    print(f"Mean: {stats['average_mean']:.6f}s, Std: {stats['average_std']:.6f}s")
    print(f"Coefficient of Variation: {stats['average_cv']:.6f}")

# Perform ANOVA test
def perform_anova():
    algorithms = {
        'Greatest Worth': greatest_worth_first['average'],
        'Largest Value': largest_value_first['average'],
        'Smallest Weight': smallest_weight_first['average']
    }
    
    f_stat, p_value = f_oneway(*algorithms.values())
    return f_stat, p_value

f_stat, p_value = perform_anova()
print("\nOne-way ANOVA Test:")
print(f"F-statistic: {f_stat:.8f}")
print(f"p-value: {p_value:.8e}")

# Dynamic Programming Time Comparison
def create_dp_time_comparison():
    fig = go.Figure()
    
    # Bottom-up trace
    fig.add_trace(go.Scatter(
        x=n_values,
        y=bottom_up_avg,
        name='Bottom-up',
        mode='lines+markers',
        marker=dict(symbol='circle', size=6),
        showlegend=False  # Remove from legend
    ))
    
    # Top-down trace
    fig.add_trace(go.Scatter(
        x=n_values,
        y=top_down_avg,
        name='Top-down Memoized',
        mode='lines+markers',
        marker=dict(symbol='square', size=6),
        showlegend=False  # Remove from legend
    ))
    
    # Add labels directly on the lines at appropriate positions
    # Find positions near the end of the data for label placement
    x_pos = n_values[-10]  # Use the position 10 from the end
    
    fig.add_annotation(
        x=x_pos,
        y=bottom_up_avg[-10],
        text="Bottom-up",
        showarrow=False,
        yshift=10
    )
    
    fig.add_annotation(
        x=x_pos,
        y=top_down_avg[-10],
        text="Top-down Memoized",
        showarrow=False,
        yshift=-10
    )
    
    fig.update_layout(
        title='Dynamic Programming Implementation Time Comparison',
        xaxis_title='Number of Items (n)',
        yaxis_title='Time (seconds)',
        showlegend=False,
        template='plotly_white'
    )
    
    return fig

# Cache Performance Visualization
def create_cache_performance():
    fig = go.Figure()
    
    # Calculate midpoints for label placement
    mid_index = len(n_values) // 2
    
    # Create stacked area plot
    fig.add_trace(go.Scatter(
        x=n_values,
        y=cache_hits,
        name='Cache Hits',
        mode='none',
        fill='tonexty',
        stackgroup='one',
        hovertemplate='Cache Hits: %{y:,.0f}<extra></extra>',
        showlegend=False
    ))
    
    fig.add_trace(go.Scatter(
        x=n_values,
        y=cache_misses,
        name='Cache Misses',
        mode='none',
        fill='tonexty',
        stackgroup='one',
        hovertemplate='Cache Misses: %{y:,.0f}<extra></extra>',
        showlegend=False
    ))
    
    # Add labels directly on the areas
    fig.add_annotation(
        x=n_values[mid_index],
        y=cache_hits[mid_index]/2,  # Place in middle of hits area
        text="Cache Hits",
        showarrow=False,
        font=dict(color='white')
    )
    
    fig.add_annotation(
        x=n_values[mid_index],
        y=cache_hits[mid_index] + cache_misses[mid_index]/2,  # Place in middle of misses area
        text="Cache Misses",
        showarrow=False,
        font=dict(color='white')
    )
    
    fig.update_layout(
        title='Cache Performance in Top-down Memoization',
        xaxis_title='Number of Items (n)',
        yaxis_title='Number of Operations',
        template='plotly_white',
        showlegend=False
    )
    
    return fig

# Create and save the plots
dp_comparison = create_dp_time_comparison()
cache_performance = create_cache_performance()

dp_comparison.write_html("dp_comparison.html")
cache_performance.write_html("cache_performance.html")

# dp_comparison.write_image("dp_comparison.png")
# cache_performance.write_image("cache_performance.png")

# note:
# applications running:
# discord
# arc browser (approx 5 tabs?)
# ghostty (terminal)
# messenger

# Extract n values
n_values = bottom_up['n'].values  # Knapsack capacities

# Calculate average optimal values from bottom_up
optimal_values = bottom_up[['value 1', 'value 2', 'value 3']].mean(axis=1).values  # Average of value 1, 2, and 3

# Average trial results for each greedy algorithm
greatest_worth_avg = [
    greatest_worth_first[['value 1', 'value 2', 'value 3']].mean(axis=1).values  # Average of value 1, 2, and 3
]

largest_value_avg = [
    largest_value_first[['value 1', 'value 2', 'value 3']].mean(axis=1).values
]

smallest_weight_avg = [
    smallest_weight_first[['value 1', 'value 2', 'value 3']].mean(axis=1).values
]

# Prepare data for plotting
# Create the plot
fig = go.Figure()

# Add trace for the average of Greatest Worth First algorithm
fig.add_trace(go.Scatter(
    x=n_values,
    y=greatest_worth_avg[0],
    mode='lines+markers',
    name='Greatest Worth First (Average)',
    line=dict(color='blue')
))

# Add trace for the average of Largest Value First algorithm
fig.add_trace(go.Scatter(
    x=n_values,
    y=largest_value_avg[0],
    mode='lines+markers',
    name='Largest Value First (Average)',
    line=dict(color='red')
))

# Add trace for the average of Smallest Weight First algorithm
fig.add_trace(go.Scatter(
    x=n_values,
    y=smallest_weight_avg[0],
    mode='lines+markers',
    name='Smallest Weight First (Average)',
    line=dict(color='orange')
))

# Add trace for average optimal values
fig.add_trace(go.Scatter(
    x=n_values,
    y=optimal_values,
    mode='lines+markers',
    name='Optimal Values (Average)',
    line=dict(color='green', dash='dash')
))

# Update layout
fig.update_layout(
    title='Comparison of Greedy Algorithms vs Average Optimal Values',
    xaxis_title='Number of Items (n)',
    yaxis_title='Average Total Value',
    template='plotly_white',
    width=1200, 
    height=800,
)

# Save the plot as an HTML file
fig.write_html("greedy_vs_optimal_comparison.html")

import scipy


def analyze_time_complexity(theoretical_time, n_values, measured_times):
    X = n_values
    
    optimal_c, _ = scipy.optimize.curve_fit(theoretical_time, X, measured_times)
    
    predicted_times = theoretical_time(X, optimal_c[0])
    
    ss_res = np.sum((measured_times - predicted_times) ** 2)
    ss_tot = np.sum((measured_times - np.mean(measured_times)) ** 2)
    r_squared = 1 - (ss_res / ss_tot)
    
    return {"c": optimal_c[0], "predicted": predicted_times, "error": r_squared}

knapsack_capacity = 1000

bottom_up_fit = analyze_time_complexity(lambda n, c: c * n  * knapsack_capacity,  bottom_up["n"], bottom_up["average"])
top_down_fit = analyze_time_complexity(lambda n, c: c * n  * knapsack_capacity,  top_down_memoized["n"], top_down_memoized["average"])

largest_value_first_fit = analyze_time_complexity(lambda n, c: c * n * np.log(n),  largest_value_first["n"], largest_value_first["average"])
greatest_worth_first_fit = analyze_time_complexity(lambda n, c: c * n * np.log(n), greatest_worth_first["n"], greatest_worth_first["average"])
smallest_weight_first_fit = analyze_time_complexity(lambda n, c: c * n * np.log(n), smallest_weight_first["n"], smallest_weight_first["average"])

algorithms = [
    {
        "title": "Bottom Up Vs Theoretical Efficiency ϴ(nW)",
        "name": "Bottom Up",
        "filename": "bottom_up_vs_theoretical.html",
        "X": bottom_up["average"],
        "predicted": bottom_up_fit["predicted"],
        "c": bottom_up_fit["c"],
        "model": "nW",
        "error": bottom_up_fit["error"]
    },
    {
        "title": "Top Down Memoized Vs Theoretical Efficiency ϴ(nW)",
        "name": "Top Down Memoized",
        "filename": "top_down_memoized_vs_theoretical.html",
        "X": top_down_memoized["average"],
        "predicted": top_down_fit["predicted"],
        "c": top_down_fit["c"],
        "model": "nW",
        "error": top_down_fit["error"]
    },
    {
        "title": "Largest Value First Vs Theoretical Efficiency O(nlogn)",
        "name": "Largest Value First",
        "X": largest_value_first["average"],
        "filename": "largest_value_first-vs-theoretical.html",
        "predicted": largest_value_first_fit["predicted"],
        "c": largest_value_first_fit["c"],
        "model": "n log n",
        "error": largest_value_first_fit["error"]
    },
    {
        "title": "Greatest Worth First Vs Theoretical Efficiency O(nlogn)",
        "name": "Greatest Worth First",
        "filename": "greatest_worth_first_vs_theoretical.html",
        "X": greatest_worth_first["average"],
        "predicted": greatest_worth_first_fit["predicted"],
        "c": greatest_worth_first_fit["c"],
        "model": "n log n",
        "error": greatest_worth_first_fit["error"]
    },
    {
        "title": "Smallest Weight First Vs Theoretical Efficiency O(nlogn)",
        "name": "Smallest Weight First",
        "filename": "smallest_weight_first_vs_theoretical.html",
        "X": smallest_weight_first["average"],
        "predicted": smallest_weight_first_fit["predicted"],
        "c": smallest_weight_first_fit["c"],
        "model": "n log n",
        "error": smallest_weight_first_fit["error"]
    }
]

for algorithm in algorithms:
    measured = algorithm["X"]
    theoretical_efficiency = algorithm["predicted"]
    c = algorithm["c"]
    r_squared = algorithm["error"]
    
    # Format the mathematical expression
    if algorithm["model"] == "nW":
        math_expression = f"T(n) = {c:.2e} × nW"
    else:
        math_expression = f"T(n) = {c:.2e} × n log n"
    
    fig = go.Figure()
    
    # Measured times
    fig.add_trace(go.Scatter(
        x=n_values,
        y=measured,
        mode='lines+markers',
        name=f'{algorithm["name"]} Running Time',
        line=dict(color='blue')
    ))
    
    # Theoretical prediction
    fig.add_trace(go.Scatter(
        x=n_values,
        y=theoretical_efficiency,
        mode='lines',
        name=f'Theoretical Model: {math_expression}',
        line=dict(color='purple', dash='dash')
    ))
    
    # Shaded area
    fig.add_trace(go.Scatter(
        x=n_values,
        y=theoretical_efficiency,
        mode='none',
        fill='tozeroy',
        fillcolor='rgba(128, 0, 128, 0.2)',
        name='Theoretical Bound',
        showlegend=False
    ))
    
    fig.update_layout(
        title={
            'text': f"{algorithm['title']}<br><sub>{math_expression} (R² = {r_squared:.4f})</sub>",
            'x': 0.5,  # Center the title horizontally
            'xanchor': 'center',
            'yanchor': 'top',
            'y': 0.95,  # Give more space at the top
            'pad': {'b': 20}  # Add padding below the title
        },
        xaxis_title='Number of Items (n)',
        yaxis_title='Running Time (seconds)',
        template='plotly_white',
        legend={
            'yanchor': "top",
            'y': 0.99,
            'xanchor': "left",
            'x': 0.01
        },
        margin={'t': 100}  # Increase top margin to accommodate title
    )

    fig.write_html(algorithm["filename"])
