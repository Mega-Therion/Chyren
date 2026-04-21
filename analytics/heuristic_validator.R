#!/usr/bin/env Rscript
# heuristic_validator.R — Bayesian Heuristic Validation for Chyren
#
# This script performs statistical analysis on Chyren's telemetry logs.
# It uses a Bayesian approach to estimate the 'convergence probability' 
# of mathematical proof searches and epistemic refinement cycles.

# Set local library path for jsonlite
.libPaths(c("~/R_libs", .libPaths()))

suppressPackageStartupMessages(library(jsonlite))

LOG_FILE <- "telemetry.log"
OUTPUT_FILE <- "state/heuristic_snapshot.json"

# Function to analyze the log
analyze_telemetry <- function() {
  if (!file.exists(LOG_FILE)) {
    return(NULL)
  }
  
  # Read JSONL log
  lines <- readLines(LOG_FILE, warn = FALSE)
  if (length(lines) == 0) return(NULL)
  
  events <- lapply(lines, function(x) {
    tryCatch(fromJSON(x), error = function(e) NULL)
  })
  events <- events[!v_is_null <- sapply(events, is.null)]
  
  if (length(events) == 0) return(NULL)
  
  # Convert to data frame
  df <- data.frame(
    timestamp = sapply(events, function(x) x$timestamp),
    component = sapply(events, function(x) x$component),
    event_type = sapply(events, function(x) x$event_type),
    payload = sapply(events, function(x) as.character(x$payload)),
    stringsAsFactors = FALSE
  )
  
  # 1. Analyze MathSpoke Tier Escalations
  math_events <- df[df$component == "MathSpoke", ]
  if (nrow(math_events) > 0) {
    success_rate <- sum(math_events$event_type == "VERIFY_SUCCESS") / 
                   (sum(math_events$event_type == "VERIFY_SUCCESS") + 
                    sum(math_events$event_type == "VERIFY_FAILURE") + 0.0001)
  } else {
    success_rate <- 0.5
  }
  
  # 2. Analyze Epistemic Mesh Entropy
  mesh_events <- df[df$component == "EpistemicMesh" & df$event_type == "MESH_ITERATION", ]
  if (nrow(mesh_events) > 0) {
    # Extract entropy values from payload using regex
    entropies <- as.numeric(gsub(".*entropy=([0-9.]+).*", "\\1", mesh_events$payload))
    avg_entropy <- mean(tail(entropies, 10), na.rm = TRUE)
    entropy_trend <- if (length(entropies) > 1) {
      diff(tail(entropies, 2))
    } else {
      0
    }
  } else {
    avg_entropy <- 0
    entropy_trend <- 0
  }
  
  # 3. Bayesian Convergence Probability
  # Prior: 0.5
  # Likelihood: based on success rate and low entropy
  # Posterior P(Convergence)
  prior <- 0.5
  likelihood <- (success_rate * (1 - avg_entropy/2)) # Simple proxy
  posterior <- (likelihood * prior) / (likelihood * prior + (1 - likelihood) * (1 - prior) + 0.0001)
  
  # Generate snapshot
  snapshot <- list(
    synthesized_at = format(Sys.time(), "%Y-%m-%dT%H:%M:%OSZ"),
    metrics = list(
      proof_success_rate = round(success_rate, 3),
      current_entropy = round(avg_entropy, 3),
      entropy_trend = round(entropy_trend, 3),
      convergence_probability = round(posterior, 4)
    ),
    verdict = if (posterior > 0.8) "STABLE" else if (posterior > 0.4) "EVOLVING" else "CRITICAL"
  )
  
  # Ensure state directory exists
  if (!dir.exists("state")) dir.create("state")
  
  # Write to file
  write(toJSON(snapshot, auto_unbox = TRUE, pretty = TRUE), OUTPUT_FILE)
  
  return(snapshot)
}

# Main loop: analyze every 30 seconds
cat("📊 CHYREN HEURISTIC VALIDATOR (R) INITIALIZED\n")
cat("Monitoring telemetry.log for Bayesian convergence analysis...\n")

while(TRUE) {
  snap <- analyze_telemetry()
  if (!is.null(snap)) {
    cat(sprintf("[%s] Convergence Prob: %.4f | Verdict: %s\n", 
                snap$synthesized_at, snap$metrics$convergence_probability, snap$verdict))
  }
  Sys.sleep(30)
}
