#python3 script.py

import hashlib
import itertools

# Define your input list (e.g., words or characters)
words = ["Icepheid", "accretion", "celestial", "penumbra", "bolide", "equinox", "terminator", "perigee", "nova", "syzygy", "orbit", "elongation", "zenith", "libration", "luna", "retrograde", "ascension", "precession", "conjunction", "sidereal", "asterism", "umbra", "azimuth", "transit", "occultation", "declination", "nadir", "apogee", "meridian", "crescent", "nebula", "photosphere", "perihelion", "pulsar", "parallax", "lunation", "solstice", "eclipse", "aphelion", "opposition", "solaris", "magnitude", "chromosphere", "heliosphere", "nutation", "gibbous", "albedo", "ecliptic", "corona", "quasar"]

# Generate all combinations of length 2 (repeat=2 for cartesian product)
# Use itertools.product for ordered combinations (apple-banana vs banana-apple)
# Otherwise use itertools.combinations for unordered unique pairs
combinations = itertools.product(words, repeat=2)

for combo in combinations:
    # Join words if needed (e.g., "applebanana")
    combined_word = ''.join(combo)
    
    # Compute SHA-256 hash
    hash_value = hashlib.sha256(combined_word.encode()).hexdigest()
    
    print(f"Word: {combined_word} | Hash: {hash_value}")   