/* This is the header file that we should generate from mod.rs
 * for consumption by C code.
 */

#ifndef COUNTER_H
#define COUNTER_H

#include <glib-object.h>

G_BEGIN_DECLS

/* FIXME: most of GTK+ assumes identifier names with a library name
 * and an object name.  So the first one here would really be
 * FOO_TYPE_COUNTER, akin to GTK_TYPE_WIDGET.
 *
 * Similarly for FOO_COUNTER(), FOO_COUNTER_CLASS(), FOO_IS_COUNTER(),
 * FOO_IS_COUNTER_CLASS(), FOO_COUNTER_GET_CLASS().
 */

#define TYPE_COUNTER            (counter_get_type ())
#define COUNTER(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), TYPE_COUNTER, Counter))
#define COUNTER_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), TYPE_COUNTER, CounterClass))
#define IS_COUNTER(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), TYPE_COUNTER))
#define IS_COUNTER_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), TYPE_COUNTER))
#define COUNTER_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CAST ((obj), TYPE_COUNTER, CounterClass))

typedef struct {
	GObject parent;
} Counter;

typedef struct {
	GObjectClass parent_class;

	guint32 (* add) (Counter *counter, guint32 v);
	guint32 (* get) (Counter *counter);
	/* set_drop_counter */
} CounterClass;

GType counter_get_type (void) G_GNUC_CONST;

Counter *counter_new (void);

guint32 counter_add (Counter *counter, guint32 v);

guint32 counter_get (Counter *counter);


G_END_DECLS

#endif
