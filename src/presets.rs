pub const CONFIG: &str = r##"# Default batteryctl config

# All the batteries to be watched by the daemon.
batteries = [
#   "BAT0"
]

# The delay between battery config reads. Too low values can affect battery ife and/or performance.
read_delay_ms = 180000

good_capacity = 75
okay_capacity = 50
bad_capacity = 20
critical_capacity = 5

# good_capacity_handler = "~/.config/batteryctl/scripts/good_capacity.sh"
# okay_capacity_handler = "~/.config/batteryctl/scripts/okay_capacity.sh"
# bad_capacity_handler = "~/.config/batteryctl/scripts/bad_capacity.sh"
# critical_capacity_handler = "~/.config/batteryctl/scripts/critical_capacity.sh"
"##;

