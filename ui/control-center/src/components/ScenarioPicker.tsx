import { useEffect, useRef, useState } from 'react';
import type { MouseEvent as ReactMouseEvent } from 'react';
import { createPortal } from 'react-dom';

export interface ScenarioPickerItem {
  id: string;
}

export interface ScenarioPickerProps {
  scenarios: ScenarioPickerItem[];
  selectedScenarioId?: string | null;
  disabled?: boolean;
  onScenarioChange?: (scenarioId: string) => void;
}

export function ScenarioPicker({
  scenarios,
  selectedScenarioId,
  disabled = false,
  onScenarioChange
}: ScenarioPickerProps) {
  const [open, setOpen] = useState(false);
  const [popoverPosition, setPopoverPosition] = useState({ left: 0, top: 0, width: 270 });
  const rootRef = useRef<HTMLDivElement | null>(null);
  const listboxRef = useRef<HTMLDivElement | null>(null);
  const selected = scenarios.find((scenario) => scenario.id === selectedScenarioId) ?? scenarios[0];
  const selectedId = selected?.id ?? '';

  useEffect(() => {
    if (!open) {
      return;
    }

    const onPointerDown = (event: MouseEvent) => {
      const target = event.target as Node;
      if (!rootRef.current?.contains(target) && !listboxRef.current?.contains(target)) {
        setOpen(false);
      }
    };

    document.addEventListener('mousedown', onPointerDown);
    return () => document.removeEventListener('mousedown', onPointerDown);
  }, [open]);

  const choose = (scenarioId: string) => {
    onScenarioChange?.(scenarioId);
    setOpen(false);
  };

  const toggleOpen = (event: ReactMouseEvent<HTMLButtonElement>) => {
    if (!open) {
      const rect = event.currentTarget.getBoundingClientRect();
      setPopoverPosition({
        left: rect.left,
        top: rect.bottom + 4,
        width: Math.max(rect.width, 270)
      });
    }
    setOpen((value) => !value);
  };

  return (
    <div className="cc-scenario-picker" ref={rootRef}>
      <button
        type="button"
        className="cc-scenario-picker-trigger"
        aria-label="Scenario"
        aria-haspopup="listbox"
        aria-expanded={open}
        disabled={disabled || scenarios.length === 0}
        title={selectedId || 'No scenarios'}
        onClick={toggleOpen}
      >
        <span>{selectedId || 'No scenarios'}</span>
        <span aria-hidden="true">⌄</span>
      </button>
      {open ? createPortal(
          <div
            ref={listboxRef}
            className="cc-scenario-picker-listbox"
            role="listbox"
            aria-label="Scenario"
            data-theme="dark"
            style={{
              left: popoverPosition.left,
              top: popoverPosition.top,
              width: `min(${popoverPosition.width}px, 60vw)`
            }}
          >
            {scenarios.map((scenario) => (
              <button
                key={scenario.id}
                type="button"
                className="cc-scenario-picker-option"
                role="option"
                aria-selected={scenario.id === selectedId}
                title={scenario.id}
                onClick={() => choose(scenario.id)}
              >
                {scenario.id}
              </button>
            ))}
          </div>,
          document.body
      ) : null}
    </div>
  );
}
