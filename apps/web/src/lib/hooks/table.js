import DateCell from '$lib/components/table/DateCell.svelte';
import IdCell from '$lib/components/table/IdCell.svelte';
import PaginationControls from '$lib/components/table/PaginationControls.svelte';
import ParagraphCell from '$lib/components/table/ParagraphCell.svelte';
import TextCell from '$lib/components/table/TextCell.svelte';
import TitleCell from '$lib/components/table/TitleCell.svelte';
import {
	columnPinningFeature,
	columnSizingFeature,
	createPaginatedRowModel,
	createTableHook,
	rowPaginationFeature,
	tableFeatures
} from '@tanstack/svelte-table';

const _features = tableFeatures({
	rowPaginationFeature,
	columnPinningFeature,
	columnSizingFeature
});

/** @typedef {typeof _features} AppTableFeatures */

const cellComponents = {
	IdCell,
	TitleCell,
	TextCell,
	ParagraphCell,
	DateCell
};

/** @typedef {typeof cellComponents} AppTableCellComponents */

const headerComponents =
	/** @type {Record<string, import('@tanstack/svelte-table').ComponentType<any>>} */ ({});

/** @typedef {typeof headerComponents} AppTableHeaderComponents */

/**
 * @template {import('@tanstack/svelte-table').RowData} T
 * @typedef {import('@tanstack/svelte-table').AppColumnHelper<AppTableFeatures, T, AppTableCellComponents, AppTableHeaderComponents>} AppTableColumnHelper
 */

const tableComponents = {
	PaginationControls
};

export const {
	createAppColumnHelper,
	createAppTable,
	useTableContext,
	useCellContext,
	useHeaderContext
} = createTableHook({
	_features,
	_rowModels: {
		paginatedRowModel: createPaginatedRowModel()
	},

	getRowId: (row) => row.id,

	cellComponents,
	headerComponents,
	tableComponents
});

/**
 * @typedef {import('@tanstack/svelte-table').Column<AppTableFeatures, any, any>} AnyAppTableColumn
 */

/**
 * @param {AnyAppTableColumn} column
 * @returns {string} A value for the `style` prop.
 */
export function getPinningStyle(column) {
	const isPinned = column.getIsPinned();

	const style = {
		bgColor: isPinned ? 'var(--color-base-100)' : 'auto',
		left: isPinned === 'left' ? `${column.getStart('left')}px` : 'auto',
		right: isPinned === 'right' ? `${column.getAfter('right')}px` : 'auto',
		opacity: isPinned ? 0.95 : 1,
		position: isPinned ? 'sticky' : 'relative',
		width: `${column.getSize()}px`,
		zIndex: isPinned ? 1 : 0
	};

	return `
    background-color: ${style.bgColor};
    left: ${style.left};
    right: ${style.right};
    opacity: ${style.opacity};
    position: ${style.position};
    width: ${style.width};
    z-index: ${style.zIndex};
	`;
}
