# opt-schedule

This should take a look at the open classes for the next college semester and figure out which ones to take for The Best Schedule™.

## Sigaa Time Format (STF)
This doesn't exactly have a name. I came up with Sigaa Time Format for variable-naming purposes. It refers to how the _Sistema Integrado de Gestão de Atividades Acadêmicas_ displays class schedules.

- A string that follows the pattern `[1-7]*[MTN][1-6]*`.
- The first digits tell the days of that class (2 for Monday, 3 for Tuesday, 4 for Wednesday...).
- The letters M, T, and N indicate whether the class is in the Morning (Manhã), Afternoon (Tarde), or Night (Noite).
- The last digits tell in which period of that part of the day the class is held. We have 6 periods in the morning and afternoon and 4 periods at night.
  > Note that, because of this, "24N56" is not a valid class schedule.

- Exemple:
  - Class A is held at 23M56. That means: Every monday and tuesday, in the morning, during the fifth and sixth periods.
 
#### About saturdays and sundays
I've never seen a class that meets on Saturdays or Sundays. I don't believe (and i don't want to believe) in their existence. They are, in spite of this, part of the STF module, because i think it would be needlessly harder to keep them out.
