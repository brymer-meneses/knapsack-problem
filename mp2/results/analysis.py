import pandas as pd
import plotly.graph_objects as go
import plotly.express as px
from plotly.subplots import make_subplots
import numpy as np
from scipy.stats import f_oneway, kruskal, mannwhitneyu
import matplotlib.pyplot as plt
from scipy import stats

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
cache_misses = top_down_memoized['average.2'].values  # Cache misses column

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

# note:
# applications running:
# discord
# arc browser (approx 5 tabs?)
# ghostty (terminal)
# messenger

def perform_optimality_analysis():
    # Calculate average values across trials
    dp_values = np.mean([bottom_up['table 1'], bottom_up['table 2'], bottom_up['table 3']], axis=0)
    algorithms = {
        'Greatest Worth First': np.mean([
            greatest_worth_first['value 1'],
            greatest_worth_first['value 2'],
            greatest_worth_first['value 3']
        ], axis=0),
        'Largest Value First': np.mean([
            largest_value_first['value 1'],
            largest_value_first['value 2'],
            largest_value_first['value 3']
        ], axis=0),
        'Smallest Weight First': np.mean([
            smallest_weight_first['value 1'],
            smallest_weight_first['value 2'],
            smallest_weight_first['value 3']
        ], axis=0)
    }
    
    # Calculate approximation ratios (optimal/greedy)
    approximation_ratios = {}
    for name, values in algorithms.items():
        # Changed to divide optimal by greedy
        ratios = dp_values / values
        approximation_ratios[name] = ratios
    
    return approximation_ratios

def analyze_greedy_optimality():
    print("\nGreedy Algorithms Optimality Analysis")
    print("=====================================")
    
    # First get the approximation ratios
    approximation_ratios = perform_optimality_analysis()
    
    # 1. Basic Statistical Summary
    print("\n1. Basic Statistical Summary")
    print("---------------------------")
    for name, ratios in approximation_ratios.items():
        print(f"\n{name}:")
        print(f"Mean Optimality: {np.mean(ratios):.6%}")
        print(f"Median Optimality: {np.median(ratios):.6%}")
        print(f"Standard Deviation: {np.std(ratios):.6%}")
        print(f"95% Confidence Interval: [{np.percentile(ratios, 2.5):.6%}, {np.percentile(ratios, 97.5):.6%}]")
    
    # 2. Create violin plot for distribution comparison
    fig = go.Figure()
    
    colors = {
        'Greatest Worth First': 'rgba(0, 128, 0, 0.7)',
        'Largest Value First': 'rgba(128, 0, 128, 0.7)',
        'Smallest Weight First': 'rgba(255, 165, 0, 0.7)'
    }
    
    for name, ratios in approximation_ratios.items():
        fig.add_trace(go.Violin(
            y=ratios,
            name=name,
            box_visible=True,
            meanline_visible=True,
            fillcolor=colors[name],
            line_color='black'
        ))
    
    fig.update_layout(
        title='Distribution of Solution Optimality by Algorithm',
        yaxis_title='Optimality Ratio',
        yaxis_tickformat=',.6%',
        template='plotly_white',
        showlegend=True,
        violinmode='group'
    )
    
    fig.write_html("optimality_distribution.html")
    
    # 3. Statistical Tests Summary
    print("\n2. Statistical Analysis")
    print("---------------------")
    
    # One-way ANOVA
    f_stat, p_value = f_oneway(*approximation_ratios.values())
    print("\nOne-way ANOVA Test:")
    print(f"F-statistic: {f_stat:.8f}")
    print(f"p-value: {p_value:.8e}")
    
    # Effect size (Eta-squared)
    def calculate_eta_squared(f_stat, groups):
        n = sum(len(group) for group in groups)
        k = len(groups)
        return (f_stat * (k-1)) / (f_stat * (k-1) + (n-k))
    
    eta_squared = calculate_eta_squared(f_stat, list(approximation_ratios.values()))
    print(f"Effect size (η²): {eta_squared:.8f}")
    
    # 4. Practical Significance Analysis
    print("\n3. Practical Significance Analysis")
    print("--------------------------------")
    
    # Calculate percentage of solutions within different optimality thresholds
    thresholds = [0.95, 0.90, 0.85, 0.80]
    
    print("\nPercentage of solutions within optimality thresholds:")
    for name, ratios in approximation_ratios.items():
        print(f"\n{name}:")
        for threshold in thresholds:
            percentage = np.mean(ratios >= threshold) * 100
            print(f"≥ {threshold:.0%} optimal: {percentage:.6f}%")
    
    # 5. Consistency Analysis
    print("\n4. Consistency Analysis")
    print("---------------------")
    
    for name, ratios in approximation_ratios.items():
        cv = np.std(ratios) / np.mean(ratios)  # Coefficient of variation
        print(f"\n{name}:")
        print(f"Coefficient of Variation: {cv:.8f}")
        print(f"Minimum Optimality: {np.min(ratios):.6%}")
        print(f"Maximum Optimality: {np.max(ratios):.6%}")
        print(f"Optimality Range: {np.max(ratios) - np.min(ratios):.6%}")

# Run the analysis
analyze_greedy_optimality()