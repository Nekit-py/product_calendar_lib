# Производственный каледнарь
## Обзор

Производственный Календарь — это инструмент, разработанный для управления и анализа дат, рабочих дней и праздников в заданный период.
Этот README содержит подробное описание функционала, инструкции по установке и примеры использования.

## Функционал
1. Выгрузка всего года: Получение производственного календаря на весь год.
2. Выгрузка за период (N календарных дней): Получение календаря с начальной даты на заданное количество календарных дней.
3. Выгрузка за период (N рабочих дней): Получение календаря с начальной даты на заданное количество рабочих дней.
4. Выгрузка за период (начальная дата - конечная дата): Получение календаря за указанный диапазон дат.
5. Статистика за период: Получение статистики о количестве дней, рабочих дней и праздников за указанный период.
6. Разбивка по кварталам: Получение календаря с разбивкой по кварталам.
7. Разбивка по неделям: Получение календаря с разбивкой по неделям.
8. Всего дней за период: Подсчет общего количества дней за указанный период.
9. Всего рабочих часов за период: Подсчет общего количества рабочих часов за указанный период.
10. Следующий рабочий день: Определение следующего рабочего дня с указанной даты.


## Установка
Для установки Производственного Календаря выполните следующие шаги:

### Самостоятельная сборка

1. Клонируйте репозиторий
```bash
https://github.com/Nekit-py/product_calendar_lib.git
```

2. Установите Maturin
https://pypi.org/project/maturin/
или
```bash
brew install maturin
```

3. Соберите библиотеку
```bash
maturin build --release
```

Для конкретной версии python:
```bash
maturin build --release --interpreter 3.10
```

Более подробно на https://github.com/pyo3/maturin



### Установка с https://www.pypi.org

1. Скачайте wheel file c https://pypi.org/project/product_calendar_lib/

2. Установите
```bash
pip install product_calendar-0.1.0-cp312-cp312-macosx_11_0_arm64.whl
```

## Использование
### Основые типы
```python
class Day:
    day: date
    weekday: str
    kind: str

    def as_dict(self) -> dict[str, str]:
        ...


class Statistic:
    def __init__(self, holidays: int, work_days: int, weekends: int, preholidays: int)):
        ...

    def work_hours(self) -> int:
        ...

    def rest_days(self) -> int:
        ...

    def as_dict(self) -> dict[str, int]:
        ...

class ProductCalendar:
    def __init__(self, year: int | None = None):
        ...

    def all_days(self) -> list[Day]:
        ...

    def period_by_number_of_days(self, date: date, days: int) -> Self:
        ...

    def period_by_number_of_work_days(self, date: date, work_days: int) -> Self:
        ...

    def period_slice(self, start: date, end: date) -> Self:
        ...

    def extract_dates_in_quarter(quarter: int) -> Self:
        ...

    def statistic(self) -> Statistic:
        ...

    def total_days(self) -> int:
        ...

    def next_work_day(self, cur_day: date) -> Day:
        ...

    def by_kind(self, kind: str) -> Self:
        ...

    def extract_dates_in_quarter(self, quarter: int) -> Self:
        ...

    def after_nth_weeks(self, date: date, weeks: int) -> Self:
        pass
```

### Выгрузка всего года
```python
from product_calendar import ProductCalendar

calendar = ProductCalendar(2024)
yearly_calendar = calendar.all_days()
print(yearly_calendar)
```

### Выгрузка за период (N календарных дней)
```python
calendar_period = calendar.period_by_number_of_days(date(2024-05-01), 10)
print(calendar_period)
```

 ### Выгрузка за период (N рабочих дней)
```python
calendar_period_working_days = calendar.period_by_number_of_work_days(date(2024-05-01), 10)
print(calendar_period_working_days)
```

 ### Выгрузка за период (начальная дата - конечная дата)
```python
calendar_period = calendar.period_slice(date(2024-05-01), date(2024-05-30))
print(calendar_period)
```

 ### Статистика за период
```python
period_statistic = calendar.statistic()
print(period_statistic)
```

 ### Разбивка по кварталам
```python
third_quarter = calendar.extract_days_in_quarter(3)
print(third_quarter)
```
### Дата спустя N недель
```python
desired_day = calendar.after_nth_weeks(3)
print(desired_day)
```

 ### Всего рабочих часов за период
```python
third_quarter = calendar.extract_days_in_quarter(3)
third_quarter_statistic = third_quarter.statistic()
print(third_quarter_statistic.work_hours())
```

 ### Следующий рабочий день
```python
desired_work_day = calendar.next_work_day(date(2024,1,1))
print(desired_day)
```


### Вклад
Мы приветствуем вклад в проект! Пожалуйста, выполните следующие шаги для внесения изменений:

Сделайте форк репозитория.
Создайте новую ветку для вашей функции или исправления ошибки.
Внесите ваши изменения.
Отправьте pull request с подробным описанием ваших изменений.

### Лицензия
Этот проект лицензирован по лицензии MIT. См. файл LICENSE для подробностей.

### Контакты
По вопросам или для поддержки свяжитесь с нами по адресу nekit-sns@yandex.ru





