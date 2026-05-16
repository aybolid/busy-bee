import DateCell from '$lib/components/table/DateCell.svelte';
import ParagraphCell from '$lib/components/table/ParagraphCell.svelte';
import TextCell from '$lib/components/table/TextCell.svelte';
import TitleCell from '$lib/components/table/TitleCell.svelte';
import { createTableHook, tableFeatures } from '@tanstack/svelte-table';

const _features = tableFeatures({});

/** @typedef {typeof _features} AppTableFeatures */

const cellComponents = {
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

export const {
	createAppColumnHelper,
	createAppTable,
	useTableContext,
	useCellContext,
	useHeaderContext
} = createTableHook({
	_features,
	_rowModels: {},

	getRowId: (row) => row.id,

	cellComponents,
	headerComponents
});
