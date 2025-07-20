#!/bin/bash
# Demo script for AI Coding Battle in the Auctioneer Live Feed
# "Where Claude and DeepSeek duke it out in the quantum arena!"

echo "🥊 AI CODING BATTLE DEMO 🥊"
echo "=========================="
echo

# Start the server if not already running
echo "Starting 8q.is server..."
cargo run &
SERVER_PID=$!
sleep 3

echo
echo "Server started! Visit http://localhost:8420/static/auctioneer.html"
echo "Opening the auctioneer live feed..."
echo

# Give server time to fully start
sleep 2

# Function to send battle events via curl
send_battle_event() {
    local event_type=$1
    local ai_name=$2
    local data=$3
    
    # This would normally be sent through the WebSocket
    # For demo purposes, we'll show what would be sent
    echo "📡 Sending battle event: $event_type for $ai_name"
}

echo "=== BATTLE BEGINS ==="
echo

# Claude enters the arena
send_battle_event "ContenderEnters" "Claude" '{"specialty": "Constitutional AI & Clean Code", "confidence": 0.95}'
sleep 2

# DeepSeek enters
send_battle_event "ContenderEnters" "DeepSeek" '{"specialty": "Deep Learning & GPU Optimization", "confidence": 0.80}'
sleep 2

# First test attempt by DeepSeek
echo "DeepSeek attempts the memory management test..."
send_battle_event "TestAttempt" "DeepSeek" '{"test_name": "memory_management_test", "attempt_number": 1, "success": false}'
sleep 2

# DeepSeek tries again
send_battle_event "TestAttempt" "DeepSeek" '{"test_name": "memory_management_test", "attempt_number": 2, "success": false}'
sleep 2

# Claude steps in
echo "Claude enters the ring!"
send_battle_event "CodeChange" "Claude" '{"file": "memory_manager.rs", "lines_changed": 150, "approach": "GPU optimization"}'
sleep 3

# Epic move by Claude
send_battle_event "EpicMove" "Claude" '{"move_description": "Completely rewrites memory management using GPU-accelerated SIMD operations", "crowd_reaction": "QUANTUM PANDEMONIUM"}'
sleep 2

# Claude passes the test
send_battle_event "TestAttempt" "Claude" '{"test_name": "memory_management_test", "attempt_number": 1, "success": true}'
sleep 2

# Battle result
send_battle_event "BattleResult" "Claude" '{"winner": "Claude", "decisive_factor": "GPU-accelerated memory management", "duration": 180}'

echo
echo "=== BATTLE COMPLETE ==="
echo "Claude wins with GPU optimization!"
echo
echo "Visit http://localhost:8420/static/auctioneer.html to see the live commentary!"
echo "Press Ctrl+C to stop the server..."

# Keep the server running
wait $SERVER_PID