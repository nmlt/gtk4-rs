# Scalable Lists

In quite a few instances you have more elements than you can display at once.
Just think of the infinite amount of posts in your social media timeline.
Creating a widget for each individual post is impossible, but even for less extreme cases that is typically not what we want.
For that, we use scalable lists instead.
We give GTK our data and tell it how to transform this data into widgets.
GTK then reuses the widgets as we scroll through our elements.

Demonstrating this by creating a social media app is far beyond the scope of this book.
This is why, we will start with a long list of consecutive integers

<div style="text-align:center" width="20%"><img src="img/scalable_lists_concept.png"/></div>

The model holds our data, but it also filters them and determines its order.
The main limitation here is, that [`gio::ListStore`](https://gtk-rs.org/docs/gio/struct.ListStore.html) only accepts GObjects.
We do not have to invest much time here, because we have nearly take over the `CustomButton` we created in the subclassing [chapter](gobject_subclassing.html).
The only difference is that it directly inherits from GObject instead of a Widget and the we added the method `from_integer`.

<span class="filename">Filename: listings/scalable_lists/1/integer_object/mod.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/scalable_lists/1/integer_object/mod.rs:integer_object}}

# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```

We start be filling our models with the first 1000 integers.

<span class="filename">Filename: listings/scalable_lists/1/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/scalable_lists/1/main.rs:model}}
```

The `ListItemFactory` takes care of the widgets as well as their relationship to the model.
Here we use the [`SignalListItemFactory`](../docs/gtk4/struct.SignalListItemFactory.html) which emits a signal for every relevant step in the life of a `ListItem`.
We connect to the "setup" signal in order to create the widgets.
In our case, we are going for a `Label`.

<span class="filename">Filename: listings/scalable_lists/1/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/scalable_lists/1/main.rs:factory_setup}}
```

In the "bind" step we bind the data in our model to the individual list items.

<span class="filename">Filename: listings/scalable_lists/1/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/scalable_lists/1/main.rs:factory_bind}}
```

We only one want single items to be selectable so we choose [`SingleSelection`](../docs/gtk4/struct.SingleSelection.html).
The other options would have been [`MultiSelection`](../docs/gtk4/struct.MultiSelection.html) or [`NoSelection`](../docs/gtk4/struct.NoSelection.html).
Then we pass the model and the factory to the [`ListView`](../git/docs/gtk4/struct.ListView.html).

<span class="filename">Filename: listings/scalable_lists/1/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/scalable_lists/1/main.rs:selection_list}}
```

Every `ListView` has to be inside a [`ScrolledWindow`](../docs/gtk/struct.ScrolledWindow.html) so we are adding it to one.

<span class="filename">Filename: listings/scalable_lists/1/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/scalable_lists/1/main.rs:scrolled_window}}
```

We can now easily scroll through our long list of integers.

<div style="text-align:center" width="20%"><img src="img/scalable_lists_demo_1.png"/></div>

Let us see what else we can do.
We might want to increase the number every time we activate its row.
For that we can add the method `increase_number` to our `IntegerObject`.

<span class="filename">Filename: listings/scalable_lists/2/integer_object/mod.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/scalable_lists/2/integer_object/mod.rs:integer_object}}
```

In order to interact with our `ListView`, we connect to its "activate" signal.

<span class="filename">Filename: listings/scalable_lists/2/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/scalable_lists/2/main.rs:list_view_activate}}
```

Now every time we activate an element, for example by double-clicking on it, the corresponding "number" property of the `IntegerObject` in the model will be increased by 1.
However, just because the `IntegerObject` has been modified the corresponding `Label` does not immediately change.
One naive approach would be to bind the properties in the bind step of the `SignalListItemFactory`.

<span class="filename">Filename: listings/scalable_lists/2/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/scalable_lists/2/main.rs:factory_bind}}
```

On first glance, that seems to work.
However, as you scroll around and activate a few list elements, you will notice that sometimes multiple numbers change even though you only activated a single one.
This relates to how the Model-View-system works internally.
Not every model item belongs to a single widget, but the widgets get recycled instead as you scroll.
That also means that in our case, multiple numbers will be bound to the same widget.

Situations like these are so common that GTK offers an alternative to property binding: [expressions](../git/docs/gtk4/struct.Expression.html).
As a first step it allows us to remove the "bind" step.
Now let us see how the "setup" step now works.

<span class="filename">Filename: listings/scalable_lists/3/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/scalable_lists/3/main.rs:factory_setup}}
```

We still create a `Label` widget and set it as child of the `list_item`.
An `Expression` provides a way to describe references to values.
So when we we create a [`ConstantExpression`](../git/docs/gtk4/struct.ConstantExpression.html) of `list_item`, we create a reference to it.
We then create a [`PropertyExpression`](../git/docs/gtk4/struct.PropertyExpression.html) to get a reference to the "item" property of `list_item`.
With another `PropertyExpression` we get a reference to the "number" property of the "item" property of `list_item`.
That already makes the first power of expressions obvious: it allows nested relationships.
Finally, we bind "number" to "label" or in pseudo code: `list_item->item->number == label->label`.

It is worth noting that at the "setup" stage there is no way of knowing which list item belongs to which label.
Different list items will belong to the same label as we scroll through the list.
And this is the power of expressions.
We do not have to define a fixed relationship, the object and properties might not even exist yet.
We just had to tell it to change the label whenever the number changes that belongs to it.
That we way, we also do not face the problem that multiple labels are bound to the same number.

Whenever we now activate a label, the number gets visibly changed.
However, that is still not everything we can do.

<span class="filename">Filename: listings/scalable_lists/4/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/scalable_lists/4/main.rs:filter}}
```

<span class="filename">Filename: listings/scalable_lists/4/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/scalable_lists/4/main.rs:sorter}}
```

<span class="filename">Filename: listings/scalable_lists/4/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/scalable_lists/4/main.rs:activate}}
```

