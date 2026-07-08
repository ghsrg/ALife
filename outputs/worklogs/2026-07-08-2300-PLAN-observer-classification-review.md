---
tags:
  - alife
  - plan
  - observer
  - classification
---

# PLAN: Виправлення за результатами огляду коду класифікатора ролей клітин

## Огляд

Було проведено детальний огляд змін у робочій директорії [feat-observer-classification](file:///C:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-observer-classification). Загальний стан коду високий, тести проходять успішно, зауваження від `clippy` відсутні. Проте виявлено два важливі аспекти, які потребують виправлення для гарантування детермінізму та повноти класифікації.

---

## Виявлені проблеми

### 1. Недетерміністичне вирішення нічиїх (Tie-Breaking)
У функціях [classify_cell_roles_potential](file:///C:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-observer-classification/src/observer/classifiers.rs#L50) та [classify_cell_roles_observed](file:///C:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-observer-classification/src/observer/classifiers.rs#L96) ітерування правил відбувається по [HashMap](file:///C:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-observer-classification/src/observer/config.rs#L34) (`config.rules`).
Оскільки порядок ітерації `HashMap` у Rust є випадковим (залежить від рандомізованого стану херування), вибір первинної ролі (`primary_label`) у випадку, коли дві ролі мають абсолютно одинаковий найвищий показник частки матеріалу, стає недетерміністичним.
Це суперечить загальним принципам детермінізму симуляції та аналітики.

### 2. Відсутність мапінгу для contractile_material в Observed класифікаторі
У функції [classify_cell_roles_observed](file:///C:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-observer-classification/src/observer/classifiers.rs#L96) зіставлення матеріалів з діями не враховує `contractile_material`.
У симуляції вже реалізовано процес [ContractileDisplacement](file:///C:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-observer-classification/src/core/process.rs#L57). Тому `contractile_material` має мапитися на `"ContractileDisplacement_executed"`.
Для `repair_material` та `sensory_material` наразі немає відповідних активних процесів у [PROCESS_REGISTRY](file:///C:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-observer-classification/src/core/process.rs#L125), тому їхній фолбек до `"unknown_action"` є очікуваним і має бути задокументований.

---

## План дій

### Крок 1. Забезпечення детермінізму (Tie-Breaking)
Модифікувати [classify_cell_roles_potential](file:///C:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-observer-classification/src/observer/classifiers.rs#L50) та [classify_cell_roles_observed](file:///C:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-observer-classification/src/observer/classifiers.rs#L96):
1. Зібрати всі правила в масив пар `(&String, &RoleRule)`.
2. Відсортувати цей масив алфавітно за назвою ролі.
3. Проводити класифікацію по відсортованому масиву правил. Це гарантує, що при рівних частках матеріалів завжди обиратиметься та сама роль (наприклад, алфавітно перша).

### Крок 2. Додавання мапінгу для контрактільних клітин
Оновити `match` у [classify_cell_roles_observed](file:///C:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-observer-classification/src/observer/classifiers.rs#L96):
```rust
        let action_feature = match rule.required_material.as_str() {
            "boundary_material" => "PassiveUptake_executed",
            "transport_material" => "ActiveUptake_executed",
            "metabolic_material" => "Metabolism_executed",
            "storage_material" => "Storage_executed",
            "synthesis_material" => "MaterialSynthesis_executed",
            "structural_material" => "Growth_executed",
            "contractile_material" => "ContractileDisplacement_executed",
            _ => "unknown_action",
        };
```

### Крок 3. Додавання коментарів про обмеження спостережень
Задокументувати в коді [classifiers.rs](file:///C:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-observer-classification/src/observer/classifiers.rs), чому `repair_material` та `sensory_material` наразі мапляться на `"unknown_action"`.

### Крок 4. Розширення тестів
Додати тестові сценарії в [phase2_observer_role_classifier.rs](file:///C:/Users/korsr/PycharmProjects/ALife/.worktrees/feat-observer-classification/tests/phase2_observer_role_classifier.rs) для перевірки:
1. Детерміністичного вибору при tie-breaking (однакові частки матеріалів).
2. Обсервації `contractile-like` ролі при наявності `ContractileDisplacement_executed > 0`.
