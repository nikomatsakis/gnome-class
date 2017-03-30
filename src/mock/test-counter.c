/* Compile with
 *   gcc -Wall -g -O0 -o test-counter test-counter.c `pkg-config --cflags --libs gobject-2.0`
 */

#include <glib-object.h>
#include "counter.h"

int
main (int argc, char **argv)
{
	Counter *counter = counter_new ();

	g_assert (IS_COUNTER (counter));

	g_assert (counter_get (counter) == 0);

	g_assert (counter_add (counter, 42) == 42);
	g_assert (counter_add (counter, 1) == 43);

	g_assert (counter_get (counter) == 43);

	g_object_unref (counter);

	return 0;
}
