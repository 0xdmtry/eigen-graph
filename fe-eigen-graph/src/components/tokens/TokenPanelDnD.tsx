"use client";
import React, {useEffect, useMemo, useState} from "react";
import {DndContext, PointerSensor, KeyboardSensor, useSensor, useSensors, closestCenter} from "@dnd-kit/core";
import {
    SortableContext,
    useSortable,
    arrayMove,
    sortableKeyboardCoordinates,
    verticalListSortingStrategy
} from "@dnd-kit/sortable";
import type {TableItem} from "@/types/operators";
import {baseTokenCards} from "@/data/tokens";
import {useToken} from "@/context/TokenContext";
import TokenCardDnD from "./TokenCardDnD";
import {TokenCardDnDType} from "@/types/tokens";

type Props = {
    tokens: Record<string, TableItem[]>;
};

export default function TokenPanelDnD({tokens}: Props) {
    const {selectedTokenSymbol, setSelectedTokenSymbol} = useToken();

    const baseMap = useMemo(() => {
        const m = new Map<string, { name: string; icon: string }>();
        baseTokenCards.forEach(t => m.set(t.symbol, {name: t.name, icon: t.icon}));
        return m;
    }, []);

    const cards = useMemo<TokenCardDnDType[]>(() => {
        const list: TokenCardDnDType[] = [];
        for (const symbol of Object.keys(tokens)) {
            const rows = tokens[symbol] || [];
            const operators = rows.length;
            const tvl = rows.reduce((s, r) => {
                try {
                    return s + BigInt(r.tvlTotalAtomic);
                } catch {
                    return s;
                }
            }, BigInt(0)).toString();
            const meta = baseMap.get(symbol) || {name: symbol, icon: ""};
            list.push({id: symbol, symbol, name: meta.name, icon: meta.icon, tvl, operators});
        }
        return list;
    }, [tokens, baseMap]);

    const symbols = useMemo(() => cards.map(c => c.symbol), [cards]);

    const [order, setOrder] = useState<string[]>([]);
    useEffect(() => {
        setOrder(prev => {
            if (!prev.length) return [...symbols].sort((a, b) => a.localeCompare(b));
            const kept = prev.filter(s => symbols.includes(s));
            const added = symbols.filter(s => !prev.includes(s)).sort((a, b) => a.localeCompare(b));
            return kept.concat(added);
        });
    }, [symbols]);

    const orderedCards = useMemo(
        () => order.map(s => cards.find(c => c.symbol === s)).filter(Boolean) as TokenCardDnDType[],
        [order, cards]
    );

    const sensors = useSensors(
        useSensor(PointerSensor, {activationConstraint: {distance: 6}}),
        useSensor(KeyboardSensor, {coordinateGetter: sortableKeyboardCoordinates})
    );

    function SortableItem({card}: { card: TokenCardDnDType }) {
        const {
            setNodeRef,
            attributes,
            listeners,
            transform,
            transition,
            setActivatorNodeRef,
            isDragging
        } = useSortable({
            id: card.id,
        });
        const style: React.CSSProperties = {
            transform: transform ? `translate3d(${transform.x}px, ${transform.y}px, 0)` : undefined,
            transition,
            opacity: isDragging ? 0.9 : 1,
        };
        return (
            <div ref={setNodeRef} style={style}>
                <TokenCardDnD
                    id={card.id}
                    symbol={card.symbol}
                    name={card.name}
                    icon={card.icon}
                    tvl={card.tvl}
                    operators={card.operators}
                    isActive={card.symbol.toUpperCase() === (selectedTokenSymbol || "").toUpperCase()}
                    onSelect={() => setSelectedTokenSymbol(card.symbol)}
                    dragHandleRef={setActivatorNodeRef as React.Ref<HTMLButtonElement>}
                    dragHandleAttributes={attributes}
                    dragHandleListeners={listeners}
                />
            </div>
        );
    }

    return (
        <div className="rounded-2xl border border-gray-200 bg-white dark:border-gray-800 dark:bg-white/[0.03]">
            <div className="border-t border-gray-100 dark:border-gray-800">
                <div className="p-4 sm:p-6">
                    <DndContext
                        sensors={sensors}
                        collisionDetection={closestCenter}
                        onDragEnd={({active, over}) => {
                            if (!over || active.id === over.id) return;
                            const oldIndex = order.indexOf(String(active.id));
                            const newIndex = order.indexOf(String(over.id));
                            if (oldIndex === -1 || newIndex === -1) return;
                            setOrder(o => arrayMove(o, oldIndex, newIndex));
                        }}
                    >
                        <SortableContext items={order} strategy={verticalListSortingStrategy}>
                            <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-3">
                                {orderedCards.map(card => (
                                    <SortableItem key={card.id} card={card}/>
                                ))}
                            </div>
                        </SortableContext>
                    </DndContext>
                </div>
            </div>
        </div>
    );
}
